use std::borrow::{Borrow, BorrowMut};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::collections::{HashMap};
use std::io::{Error, Read, Write};
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::mpsc::{Receiver, TryRecvError};
use macroquad::input::{get_char_pressed, get_last_key_pressed, mouse_position};
use macroquad::prelude::{clear_background, is_mouse_button_pressed};
use macroquad::window::next_frame;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::{KeyCode, MouseButton, ServerEvent};
use crate::network::Mode::Platform1;

const PORT: &str = "55555";

// TODO: hvis client og server fps ikke er synca vil vel buffer fylle seg opp

pub struct Waiter<ServerData: Default + Serialize + DeserializeOwned + Send + 'static> {
    r_stream: Receiver<TcpStream>,
    t_waiter: JoinHandle<()>,
    _lol: PhantomData<ServerData> // TODO: få api til å funke uten denne rare variabelen
}

impl<ServerData> Waiter<ServerData>
    where ServerData: Default + Serialize + DeserializeOwned + Send + 'static {
    pub fn try_get(self) ->Result<Client<ServerData>, Self> {
        
        // TODO: burde være mulig å lage denne funksjonen uten at den lager errors med self osv.
        // thread handle kan ikke joines i en loop,
        
        match self.r_stream.try_recv(){
            Ok(stream) => {
                self.t_waiter.join().unwrap();
    
                // connection threads
                println!(
                    "connection established, \n\tfrom {} \n\tto {}",
                    stream.local_addr().unwrap(),
                    stream.peer_addr().unwrap(),
                );
    
                // TODO: litt weird at det viktigste thread koden ligger her
                let mut stream_out = stream.try_clone().unwrap();
                let (s_out, r_out) = mpsc::channel();
                thread::Builder::new().name("client, out".to_string()).spawn(move || loop {
                    let event: PlayerEvent = r_out.recv().unwrap();
                    stream_out.write_all(
                        &bincode::serialize(&event).unwrap()
                    ).unwrap();
                }).unwrap();
    
                // TODO: litt weird at det viktigste thread koden ligger her også
                let mut stream_in = stream.try_clone().unwrap();
                let (s_in, r_in) = mpsc::channel();
                thread::Builder::new().name("client, in".to_string()).spawn(move || {
                    let mut buffer = [0u8; 512];
                    loop {
                        stream_in.read(&mut buffer).unwrap();
                        let event: ServerData = bincode::deserialize(&buffer).unwrap();
                        s_in.send(event).unwrap();
                    }
                }).unwrap();
    
                Ok(Client {
                    s_out,
                    r_in,
                    last_data: ServerData::default()
                })
            }
            Err(_) => Err(self)
        }
    }
}

pub struct Client<ServerData: Serialize + DeserializeOwned> {
    s_out: mpsc::Sender<PlayerEvent>,
    r_in: mpsc::Receiver<ServerData>,
    last_data: ServerData,
}

impl<ServerData> Client<ServerData>
    where ServerData: Serialize + DeserializeOwned + Default + Send + 'static {
    pub fn connect_to(ip: &str) -> Waiter<ServerData> {
    
        // TODO: bytte param:ip til std::net::IpAddrV4
    
        // connect to server
        let ip = chain(ip, ":", PORT);
        let (sender, receiver) = mpsc::channel();
        let sender_clone = sender.clone();
        let waiter = thread::Builder::new().name("client, waiter".to_string()).spawn(move ||
            loop {
                if let Ok(stream) = TcpStream::connect(&ip) {
                    sender_clone.send(stream).unwrap();
                    break;
                }
                thread::sleep(Duration::from_secs(3));
            }
        ).unwrap();
        
        return Waiter {
            r_stream: receiver,
            t_waiter: waiter,
            _lol: Default::default()
        };
    }
    
    pub fn send_input(&mut self, get_input: fn()->Option<PlayerEvent>){
        
        if let Some(e) = get_input(){
            
            self.s_out.send(e).unwrap();
        }
        
        // while let Some(e) = get_input() {
        //
        //     self.s_out.send(e).unwrap()
        // }
    }
    
    pub fn get_game_state(&mut self) -> &ServerData {
        
        // TODO: hvis de ikke er perfekt synca, vil channel fylle seg opp?
        
        if let Ok(data) = self.r_in.try_recv(){
            
            self.last_data = data;
        }

        &self.last_data
    }
}

#[derive(Debug)]
pub enum ServerErr{
    Opening(String, Box<dyn std::error::Error>),
    
}

impl std::fmt::Display for ServerErr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerErr::Opening(ip, err) =>
                write!(f, "could not open server on {ip}, internal error: {err}")
        }
    }
}

impl std::error::Error for ServerErr{

}

pub fn start_server<T>(ip: &str, server_loop: fn(&mut Kjetil<T>))->Result<(), ServerErr>
                             where T: Serialize + DeserializeOwned + Send + 'static {
    
    // TODO: bytte param:ip til std::net::IpAddrV4
    
    let full_ip = chain(ip, ":", PORT);
    
    // åpne forbindelse
    let listener = match TcpListener::bind(full_ip){
        Ok(t)=>t,
        Err(e)=> return Err(ServerErr::Opening(ip.to_string(), e.into())),
    };
    
    thread::Builder::new().name("server, loop".to_string()).spawn(move || {
        
        // lytter hele tiden til nye clients, server loop må adde dem
        let (s_stream, r_stream) = mpsc::channel();
        thread::Builder::new().name("server, listener".to_string()).spawn(move || {
            for stream in listener.incoming() {
                let s = stream.unwrap();
                s_stream.send(s).expect("server listener: receiver was disconnected");
            }
        }).unwrap();
        
        let mut server_data = Kjetil {
            connections: Vec::<Connection<T>>::new(),
            player_inputs: Vec::<PlayerInput>::new(),
            r_stream,
        };
        
        server_loop(&mut server_data);
    }).expect("server could not start thread, not enough system resources?");
    
    Ok(())
}

pub struct Kjetil<ServerData: Serialize + DeserializeOwned + Send + 'static> {
    connections: Vec<Connection<ServerData>>,
    player_inputs: Vec<PlayerInput>,
    r_stream: mpsc::Receiver<TcpStream>,
}

impl<ServerData> Kjetil<ServerData> where ServerData: Serialize + DeserializeOwned + Send + 'static + Clone {
    pub fn update_and_get_input(&mut self) -> &mut Vec<PlayerInput> {
    
        match self.r_stream.try_recv() {
            Ok(stream) => {
                println!("serverdata receiver found connection");
                self.connections.push(Connection::new(stream));
                self.player_inputs.push(PlayerInput::new());
            }
            Err(e) => match e {
                TryRecvError::Empty => {}
                TryRecvError::Disconnected => {
                    panic!("serverdata receiver error: channel was disconnected: {e}");
                }
            }
        }
        
        // oppdater data til api-objektet "player_inputs:PlayerInput"
        for (connection, player)
        in self.connections.iter_mut().zip(self.player_inputs.iter_mut()) {
            player.buffer.clear();
            
            while let Some(t) = connection.receive() {
                
                player.buffer.push(t);
            }
        }
        
        // // kanskje listener har funnet en ny connection
        // if let Ok(t) = self.r_stream.try_recv() {
        //     self.connections.push(Connection::new(t));
        //     self.player_inputs.push(PlayerInput::new());
        // }
        
        &mut self.player_inputs
    }
    
    pub fn send_game_state(& self, state: & ServerData){
        
        for i in self.connections.iter(){
            i.sender_game_state.send(state.clone()).unwrap();
        }
    }
}

struct Connection<T: Serialize + DeserializeOwned + Send + 'static> {
    thread_event: JoinHandle<()>,
    thread_game_state: JoinHandle<()>,
    receiver_event: mpsc::Receiver<PlayerEvent>,
    sender_game_state: mpsc::Sender<T>,
}

impl<T> Connection<T> where T: Serialize + DeserializeOwned + Send + 'static {
    fn new(stream: TcpStream) -> Connection<T> {
        
        println!("server starting up a new connection");
        
        let mut stream_event = stream.try_clone().unwrap();
        let (sender_event, receiver_event) = mpsc::channel();
        let thread_event = thread::Builder::new().name("server connection event".to_string()).spawn(move || {
            let mut buffer = [0; 512];
            
            loop {
                let size = stream_event.read(&mut buffer).unwrap();
                
                sender_event.send(
                    bincode::deserialize(&buffer[..size]).unwrap()
                ).expect("server connection receiever disconnected");
            }
        }).unwrap();
        
        let mut stream_game_state = stream.try_clone().unwrap();
        let (sender_game_state, receiver_game_state) = mpsc::channel();
        let thread_game_state = thread::Builder::new().name("server connection game state".to_string()).spawn(move || loop {
            let event: T = receiver_game_state.recv().unwrap();
            
            stream_game_state.write_all(
                &bincode::serialize(&event).unwrap()
            ).unwrap();
        }).unwrap();
        
        Connection {
            thread_event,
            thread_game_state,
            receiver_event,
            sender_game_state,
        }
    }
    
    fn receive(&mut self) -> Option<PlayerEvent> {
        match self.receiver_event.try_recv() {
            Ok(t) => Some(t),
            _ => None,
        }
    }
    
    fn send(&mut self, server_event: T) {
        self.sender_game_state.send(server_event).unwrap();
    }
}

impl<T> Drop for Connection<T> where T: Serialize + DeserializeOwned + Send + 'static {
    fn drop(&mut self) {
        todo!() // lukke threads
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Keys {
    Up,
    Down,
    Right,
    Left,
    W,
    A,
    S,
    D,
    Space,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum PlayerEvent {
    Pressed(Keys),
    Released(Keys),
    MouseRight(f32, f32),
    MouseLeft(f32, f32),
    MouseRightReleased,
    MouseLeftReleased,
    Unknown,
}

pub struct PlayerInput {
    buffer: Vec<PlayerEvent>, // TODO: dette burde kanskje være en queue
    downs: HashMap<Keys, bool>,
}

impl PlayerInput {
    fn new() -> PlayerInput {
        PlayerInput {
            buffer: Vec::new(),
            downs: HashMap::new(),
        }
    }
    
    fn update_map(&mut self) {
        self.buffer.retain(|x|{
            
            match x{
                PlayerEvent::Pressed(k)=>{self.downs.insert(*k, true);},
                PlayerEvent::Released(k)=>{self.downs.insert(*k, false);},
                _=>{}
            }
            false
        });
    }
    
    pub fn key_is_down(&mut self, key_code: Keys) -> bool {
        
        self.update_map();
        
        match self.downs.get(&key_code){
            Some(t)=>*t,
            None=>false
        }
    }
    
    // pub fn key_pressed(&mut self, key_code: Keys) -> bool {
    //
    //     println!("{:?}", self.buffer);
    //
    //     self.buffer.retain(|x| matches!(x, PlayerEvent::Pressed(key_code)));
    //
    //     let mut index: Option<usize> = None;
    //
    //     for (i, event) in self.buffer.iter().enumerate() {
    //         if matches!(event, PlayerEvent::Pressed(key_code)) {
    //             index = Some(i);
    //             break;
    //         }
    //     }
    //
    //     if let Some(i) = index {
    //         self.buffer.remove(i);
    //         return true;
    //     }
    //     false
    // }
}

#[derive(PartialEq)]
pub enum Mode {
    /*Menu, */Lobby,
    Platform1,
    Quit,
    Hjornefotball,
}

pub fn chain(a: &str, b: &str, c: &str) -> String {
    let mut abc = String::new();
    abc.push_str(a);
    abc.push_str(b);
    abc.push_str(c);
    abc
}
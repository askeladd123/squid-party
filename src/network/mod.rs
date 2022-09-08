use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::collections::{HashMap};
use std::io::{Read, Write};
use std::sync::mpsc::TryRecvError;
use macroquad::input::{get_char_pressed, get_last_key_pressed, mouse_position};
use macroquad::prelude::is_mouse_button_pressed;
use macroquad::window::next_frame;
use crate::{KeyCode, MouseButton};
use crate::network::Mode::Platform1;

/*

startup
* client/server: connect...

loop
* client: get player input - send input data to server
- server: convert input data to api object PlayerInput
- server: route PlayerInput to corresponding Game Mode logic function
- server: corresponding Game Mode logic function returns UserData
- server: serialize UserData and send to client
- client: deserialize, route and display

sending system: enum MetaData + struct UserData

*/

pub async fn client_loop(){
    
    // connect to server
    let (sender, receiver) = mpsc::channel();
    let waiter = thread::Builder::new().name("client, waiter".to_string()).spawn(||{
        
        // TODO: riktig addresse
        let mut stream = TcpStream::connect("lol").unwrap();
        sender.send(stream).unwrap();
    }).unwrap();
    
    // waiting for connection
    while let Err(TryRecvError::Empty) = receiver.try_recv(){
        
        // TODO: wating screen
        println!("waiting for connection");
    }
    
    let mut stream = receiver.recv().unwrap();

    waiter.join().unwrap();
    
    // connection threads
    println!("connection established, (hopefully)");
    
    let (s_out, r_out) = mpsc::channel();
    thread::Builder::new().name("client, out".to_string()).spawn(move||{
        
        loop{
            stream.write_all(r_out.recv().unwrap()).unwrap();
        }
    }).unwrap();
    
    let (s_in, r_in) = mpsc::channel();
    thread::Builder::new().name("client, in".to_string()).spawn(move||loop{
    
    }).unwrap();
    
    loop{
        // get keys
        {
            // flere keys kan trykkes samtidig
            while let Some(k) = get_last_key_pressed() {
    
                use PlayerEvent::*;
                use Keys::*;
                let event: PlayerEvent = match k{
                    KeyCode::Up=>Pressed(Up),
                    KeyCode::Down=>Pressed(Down),
                    KeyCode::Right=>Pressed(Right),
                    KeyCode::Left=>Pressed(Left),
                    KeyCode::W=>Pressed(W),
                    KeyCode::A=>Pressed(A),
                    KeyCode::S=>Pressed(S),
                    KeyCode::D=>Pressed(D),
                    KeyCode::Space=>Pressed(Space),
                    _=>PlayerEvent::Unknown,
                };
                
                // fyller opp channel buffer med player events
                s_out.send(&bincode::serialize(&event).unwrap()).unwrap();
            }
            
            if is_mouse_button_pressed(MouseButton::Left){
                s_out.send(
                    &bincode::serialize(
                        &PlayerEvent::MouseLeft(
                            mouse_position().0,
                            mouse_position().1)
                    ).unwrap()
                ).unwrap();
            }
    
            if is_mouse_button_pressed(MouseButton::Right){
                s_out.send(
                    &bincode::serialize(
                        &PlayerEvent::MouseRight(
                            mouse_position().0,
                            mouse_position().1)
                    ).unwrap()
                ).unwrap();
            }
        }
        
        next_frame().await;
    }
}

pub async fn start_server(){
    
    // åpne forbindelse
    let listener =
        TcpListener::bind(
            local_ip_address::local_ip().unwrap()
                .to_string()).unwrap();
    
    thread::Builder::new().name("server, loop".to_string()).spawn(move||{
        
        
        // lytter hele tiden til nye clients, server loop må adde dem
        let (s_stream, r_stream) = mpsc::channel();
        thread::Builder::new().name("server, listener".to_string()).spawn(move||{
            for stream in listener.incoming(){
                s_stream.send(
                    stream.unwrap()
                ).unwrap();
            }
        }).unwrap();
        
        let mut server_data = ServerData{
            connections: Vec::<Connection>::new(),
            player_inputs: Vec::<PlayerInput>::new(),
            r_stream,
        };

        server_loop(server_data);

    }).unwrap();
}


// placeholder
fn server_loop(server_data: ServerData)->ServerEvent{

    // kjør matching med gamemodes, route data fra forskjellige game modes
    server_data.update_and_get();
}

struct ServerData{
    connections:Vec<Connection>,
    player_inputs:Vec<PlayerInput>,
    r_stream: mpsc::Receiver<TcpStream>,
}

impl ServerData {
    pub fn update_and_get(&mut self)->&Vec<PlayerInput>{

        // oppdater data til api-objektet "player_inputs:PlayerInput"
        for (connection, player) in
        self.connections.iter_mut()
            .zip(self.player_inputs.iter_mut()){
            player.events.clear();

            while let Some(t) = connection.receive(){
                player.events.push(t);
            }
        }

        // kanskje listener har funnet en ny connection
        if let Ok(t) = r_stream.try_recv(){
            connections.push(Connection::new(t));
            player_inputs.push(PlayerInput::new());
        }

        self.player_inputs
    }
}

enum ServerEvent{
    Lobby(_),
    Platform1(_),
}

struct Connection{
    thread_event: JoinHandle<()>,
    thread_game_state: JoinHandle<()>,
    receiver_event: mpsc::Receiver<PlayerEvent>,
    sender_game_state: mpsc::Sender<()>,
}

impl Connection{
    fn new(stream: TcpStream)->Connection{
        
        let mut stream_event = stream.try_clone().unwrap();
        let (sender_event, receiver_event) = mpsc::channel();
        let thread_event = thread::Builder::new().name("server connection event".to_string()).spawn(move||{
            
            let mut buffer = [0;512];
 
            loop {
                
                let size = stream_event.read(&mut buffer).unwrap();
                
                sender_event.send(
                    bincode::deserialize(&buffer[..size]).unwrap()
                ).unwrap();
            }
        }).unwrap();
    
        let mut stream_game_state = stream.try_clone().unwrap();
        let (sender_game_state, receiver_game_state) = mpsc::channel();
        let thread_game_state = thread::Builder::new().name("server connection game state".to_string()).spawn(move||{
            todo!()
        }).unwrap();
    
        Connection{
            thread_event,
            thread_game_state,
            receiver_event,
            sender_game_state,
        }
    }
    
    fn receive(&mut self)->Option<PlayerEvent>{
        match self.receiver_event.try_recv(){
            Ok(t)=>Some(t),
            _=>None,
        }
    }
    
    fn send(&mut self){
        todo!()
    }
}

impl Drop for Connection{
    fn drop(&mut self) {
        todo!()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Keys{
    Up, Down, Right, Left,
    W, A, S, D,
    Space,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum PlayerEvent{
    Pressed(Keys),
    Released(Keys),
    MouseRight(f32, f32),
    MouseLeft(f32, f32),
    MouseRightReleased,
    MouseLeftReleased,
    Unknown,
}

pub struct PlayerInput{
    events: Vec<PlayerEvent>,
}

impl PlayerInput{
    fn new()->PlayerInput{
        PlayerInput{
            events: Vec::new(),
        }
    }
 
    pub fn key_pressed(&mut self, key_code: Keys)->bool{
        
        self.events.retain(|x|matches!(x, PlayerEvent::Pressed(key_code)));
        
        let mut index:Option<usize> = None;
        
        for (i, event) in self.events.iter().enumerate(){
            if matches!(event, PlayerEvent::Pressed(key_code)){
                index = Some(i);
                break;
            }
        }
        
        if let Some(i) = index{
            self.events.remove(i);
            return true;
        }
        false
    }
}

#[derive(PartialEq)]
pub enum Mode {
    /*Menu, */Lobby, Platform1, Quit, Hjornefotball
}
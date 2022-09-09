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

pub async fn start_client() {
    
    // connect to server
    let (sender, receiver) = mpsc::channel();
    let waiter = thread::Builder::new().name("client, waiter".to_string()).spawn(|| {
        
        // TODO: riktig addresse
        let mut stream = TcpStream::connect("lol").unwrap();
        sender.send(stream).unwrap();
    }).unwrap();
    
    // waiting for connection
    while let Err(TryRecvError::Empty) = receiver.try_recv() {
        
        // TODO: wating screen
        println!("waiting for connection");
    }
    
    let mut stream = receiver.recv().unwrap();
    
    waiter.join().unwrap();
    
    // connection threads
    println!("connection established, (hopefully)");
    
    let mut stream_out = stream.try_clone().unwrap();
    let (s_out, r_out) = mpsc::channel();
    thread::Builder::new().name("client, out".to_string()).spawn(move || loop {
    
        let event: PlayerEvent = r_out.recv().unwrap();
        stream_out.write_all(
            &bincode::serialize(&event).unwrap()
        ).unwrap();
    
    }).unwrap();
    
    let mut stream_in = stream.try_clone().unwrap();
    let (s_in, r_in) = mpsc::channel();
    thread::Builder::new().name("client, in".to_string()).spawn(move || {
        
        let mut buffer = [0u8;512];
        stream_in.read(&mut buffer).unwrap();
        
        let event:ServerEvent = bincode::deserialize(&buffer).unwrap();
        s_in.send(event).unwrap();
        
    }).unwrap();
    
    let mut client_data = ClientData {
        s_out,
        r_in,
    };
    
    client_loop(client_data);
    }
}

fn client_loop(client_loop: ClientData) {



}

struct ClientData {
    s_out:mpsc::Sender<PlayerEvent>,
    r_in:mpsc::Receiver<ServerEvent>,
}

impl ClientData {
    fn update_and_get_input(&mut self) {
        // flere keys kan trykkes samtidig
        while let Some(k) = get_last_key_pressed() {
            use PlayerEvent::*;
            use Keys::*;
            let event: PlayerEvent = match k {
                KeyCode::Up => Pressed(Up),
                KeyCode::Down => Pressed(Down),
                KeyCode::Right => Pressed(Right),
                KeyCode::Left => Pressed(Left),
                KeyCode::W => Pressed(W),
                KeyCode::A => Pressed(A),
                KeyCode::S => Pressed(S),
                KeyCode::D => Pressed(D),
                KeyCode::Space => Pressed(Space),
                _ => PlayerEvent::Unknown,
            };
        
            // fyller opp channel buffer med player events
            self.s_out.send(event).unwrap();
        }
    
        if is_mouse_button_pressed(MouseButton::Left) {
            self.s_out.send(PlayerEvent::MouseLeft(
                        mouse_position().0,
                        mouse_position().1)
                ).unwrap()
        }
    
        if is_mouse_button_pressed(MouseButton::Right) {
            self.s_out.send(
                PlayerEvent::MouseRight(
                        mouse_position().0,
                        mouse_position().1)
                ).unwrap();
        }
    }
    
}

pub async fn start_server() {
    
    // åpne forbindelse
    let listener = TcpListener::bind(
        local_ip_address::local_ip().unwrap().to_string()).unwrap();
    
    thread::Builder::new().name("server, loop".to_string()).spawn(move || {
        
        // lytter hele tiden til nye clients, server loop må adde dem
        let (s_stream, r_stream) = mpsc::channel();
        thread::Builder::new().name("server, listener".to_string()).spawn(move || {
            for stream in listener.incoming() {
                s_stream.send(
                    stream.unwrap()
                ).unwrap();
            }
        }).unwrap();
        
        let mut server_data = ServerData {
            connections: Vec::<Connection>::new(),
            player_inputs: Vec::<PlayerInput>::new(),
            r_stream,
        };
        
        server_loop(&mut server_data);
    }).unwrap();
}


// placeholder fn server_loop(server_data: &mut ServerData) {
    
    // kjør matching med gamemodes, route data fra forskjellige game modes
    
    let mut mode = Mode::Lobby;
    
    return match mode {
        Lobby => {
            let input = server_data.update_and_get_input();
            // lobby::logic() goes here
            // server_data.connections.iter().send(b)
            todo!();
        }
        Platform1 => {
            let input = server_data.update_and_get_input();
            todo!()
        }
        Quit => {
            let input = server_data.update_and_get_input();
            todo!()
        }
        Hjornefotball => {
            let input = server_data.update_and_get_input();
            todo!()
        }
    };
}

struct ServerData {
    connections: Vec<Connection>,
    player_inputs: Vec<PlayerInput>,
    r_stream: mpsc::Receiver<TcpStream>,
}

impl ServerData {
    pub fn update_and_get_input(&mut self) -> &Vec<PlayerInput> {
        
        // oppdater data til api-objektet "player_inputs:PlayerInput"
        for (connection, player) in self.connections.iter_mut().zip(self.player_inputs.iter_mut()) {
            player.events.clear();
            
            while let Some(t) = connection.receive() {
                player.events.push(t);
            }
        }
        
        // kanskje listener har funnet en ny connection
        if let Ok(t) = self.r_stream.try_recv() {
            self.connections.push(Connection::new(t));
            self.player_inputs.push(PlayerInput::new());
        }
        
        &self.player_inputs
    }
}

enum ServerEvent {
    Lobby(_),
    Platform1(_),
}

struct Connection {
    thread_event: JoinHandle<()>,
    thread_game_state: JoinHandle<()>,
    receiver_event: mpsc::Receiver<PlayerEvent>,
    sender_game_state: mpsc::Sender<ServerEvent>,
}

impl Connection {
    fn new(stream: TcpStream) -> Connection {
        let mut stream_event = stream.try_clone().unwrap();
        let (sender_event, receiver_event) = mpsc::channel();
        let thread_event = thread::Builder::new().name("server connection event".to_string()).spawn(move || {
            let mut buffer = [0; 512];
            
            loop {
                let size = stream_event.read(&mut buffer).unwrap();
                
                sender_event.send(
                    bincode::deserialize(&buffer[..size]).unwrap()
                ).unwrap();
            }
        }).unwrap();
        
        let mut stream_game_state = stream.try_clone().unwrap();
        let (sender_game_state, receiver_game_state) = mpsc::channel();
        let thread_game_state =
            thread::Builder::new()
                .name("server connection game state".to_string())
                .spawn(move || loop {
            let event: ServerEvent = receiver_game_state.recv().unwrap();
            
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
    
    fn send(&mut self, server_event: ServerEvent) {
        self.sender_game_state.send(server_event).unwrap();
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        todo!()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
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

#[derive(serde::Serialize, serde::Deserialize)]
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
    events: Vec<PlayerEvent>,
}

impl PlayerInput {
    fn new() -> PlayerInput {
        PlayerInput {
            events: Vec::new(),
        }
    }
    
    pub fn key_pressed(&mut self, key_code: Keys) -> bool {
        self.events.retain(|x| matches!(x, PlayerEvent::Pressed(key_code)));
        
        let mut index: Option<usize> = None;
        
        for (i, event) in self.events.iter().enumerate() {
            if matches!(event, PlayerEvent::Pressed(key_code)) {
                index = Some(i);
                break;
            }
        }
        
        if let Some(i) = index {
            self.events.remove(i);
            return true;
        }
        false
    }
}

#[derive(PartialEq)]
pub enum Mode {
    /*Menu, */Lobby,
    Platform1,
    Quit,
    Hjornefotball,
}

fn chain(a: &str, b: &str, c: &str)->String {
    let mut abc = String::new();
    abc.push_str(a);
    abc.push_str(b);
    abc.push_str(c);
    abc
}
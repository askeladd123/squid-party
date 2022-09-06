use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::collections::{HashMap};

// handle player input - send to logic loop
// serialize then send to graphics
// connection threads

fn logic<GameState>(player_events:()) ->(Mode, GameState) where GameState: Serialize + Deserialize{
    todo!()
}

fn graphics<GameState>(game_state:GameState){
    todo!()
}

fn start_server(){

    thread::Builder::new().name("server loop".to_string()).spawn(move||{

        thread::Builder::new().name("server listener".to_string()).spawn(mode||{
            // new connections
        }).unwrap();

        let mut mode = Mode::Lobby;
        loop{
            match mode{
                Mode::Lobby => {

                }
                Mode::Platform1 => {}
                Mode::Quit => {}
                Mode::Hjornefotball => {}
            }

            thread::sleep(Duration::from_millis(16));
        }
    }).unwrap();

}

struct Connection{
    thread_event: JoinHandle<()>,
    thread_game_state: JoinHandle<()>,
    receiver_event: mpsc::Receiver<()>,
    sender_game_state: mpsc::Sender<()>,
}

impl Connection{
    fn new(stream: TcpStream)->Connection{
        
        let (sender_event, receiver_event) = mpsc::channel();
        let thread_event = thread::Builder::new().name("server connection event".to_string()).spawn(move||{
            todo!()
        }).unwrap();
    
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
    
    fn send(&mut self){
        todo!()
    }
}

impl Drop for Connection{
    fn drop(&mut self) {
        todo!()
    }
}

pub struct PlayerInput{
    keys:HashMap<u8, bool>,
    pos: (f32, f32),
}

impl PlayerInput{
    pub fn is_key_pressed(&mut self, key_code: u8)->bool{
        let a = self.keys.get_mut(&key_code).expect("keycode not implemented");
        let b = *a;
        *a = false;
        b
    }
}

#[derive(PartialEq)]
pub enum Mode {
    /*Menu, */Lobby, Platform1, Quit, Hjornefotball
}
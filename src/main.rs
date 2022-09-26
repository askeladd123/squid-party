mod lobby;
mod menu;
mod player;
mod platform1;
mod common;
mod hjornefotball;
mod network;

use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::Deref;
use std::time::Duration;
use macroquad::prelude::*;
use common::input;
use player::Player;
use crate::common::MenuMode;
use crate::network::{Client, Keys, PlayerEvent, Kjetil, Waiter};

// TODO flytte kode ut i andre filer (så man kan samarbeide)

#[macroquad::main("BetaDev")]
async fn main() {
    let mut common_data = common::Data::new(common::files::Data::new().await);
    
    loop {
        match common_data.mode {
            MenuMode::Quit => break,
            MenuMode::Main => {
                let mut menu_data = menu::Data::new();
                
                while common_data.mode == MenuMode::Main {
                    menu::tick(&mut menu_data, &mut common_data);
                    next_frame().await;
                }
            }
            MenuMode::SinglePlayer => {
                // når spillet er ferdig går du tilbake til menyen
                common_data.mode = MenuMode::Main;
                
                // init
                network::start_server("localhost", server_loop);
                
                // loop
                let mut waiter = network::Client::<ServerEvent>::connect("localhost");
                let mut client = loop {
                    // TODO: connecting screen
                    println!("client waiting for connection");
                    
                    waiter = match waiter.try_get() {
                        Ok(t) => break t,
                        Err(s) => s,
                    };
                    
                    next_frame().await;
                };
                
                println!("client found connection bro");
                
                // TODO: flytte loopen inn i client, så du kan deklarere variabler lettere?
                
                loop {
                    match client.get_game_state() {
                        ServerEvent::Lobby(_) => {
                            
                            let mut lobby_data = lobby::Data::new();
                            
                            while let ServerEvent::Lobby(ref data) = client.get_game_state() {
                                
                                // lobby::tick(&mut lobby_data, &mut common_data, &mut players);
                                lobby::client(&common_data.files, data);
                                
                                client.send_input(input_macroquad);
                                
                                next_frame().await;
                            }
                        }
                        ServerEvent::Platform1 => {
                            todo!();
                        }
                        ServerEvent::Hjornefotball => {
                            todo!();
                        }
                    }
                }
            }
            MenuMode::Options => {}
            MenuMode::MultiPlayer => {
                while common_data.mode == MenuMode::MultiPlayer {
                    todo!() // send til enten host eller join
                }
            }
            MenuMode::Join => {
                let mut ip = std::rc::Rc::new(String::new());
                let mut written = Box::new(false);
                while common_data.mode == MenuMode::Join {
                    clear_background(BLACK);
                    
                    egui_macroquad::ui(|egui_ctx| {
                        egui_macroquad::egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                            ui.label("type in server ip: ");
                            ui.text_edit_singleline(&mut ip.to_string());
                            if ui.button("enter").clicked() {
                                // TODO: sjekk om text matcher en ip
                                *written = true;
                            };
                        });
                    });
                    egui_macroquad::draw();
                    
                    if *written {
                        // network::start_client((*ip).clone(), client_loop).await;
                    }
                }
            }
            MenuMode::Host => {
                // common_data.mode == MenuMode::Main;
                //
                // let ip = local_ip_address::local_ip().unwrap().to_string();
                // network::start_server(ip.clone(), server_loop).await;
                // network::start_client(ip.clone(), client_loop).await;
            }
            
            /*
            MenuMode::Menu=> {
                let mut menu_data = menu::Data::new();
                
                while common_data.mode == MenuMode::Menu {
                    menu::tick(&mut menu_data, &mut common_data);
                    next_frame().await;
                }
            },
    
            MenuMode::Lobby=> {
                let mut lobby_data = lobby::Data::new();

                while common_data.mode == MenuMode::Lobby{
                    lobby::tick(&mut lobby_data, &mut common_data, &mut players);
                    next_frame().await;
                }
            },
    
            MenuMode::Platform1 => {
                let mut platform1_data = platform1::Data::new();
                
                while common_data.mode == MenuMode::Platform1 {

                    platform1::tick(&mut platform1_data);
                    next_frame().await;
                }
            },
            MenuMode::Hjornefotball=> {
                let mut hjornefotball_data = hjornefotball::Data::new();

                while common_data.mode == MenuMode::Hjornefotball{
                    hjornefotball::tick(&mut hjornefotball_data, &mut common_data);
                    next_frame().await;
                }
            },
            
             */
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
enum ServerEvent {
    Lobby(lobby::State),
    Platform1/*(platform1::State)*/,
    Hjornefotball/*(hjornefotball::State)*/,
}

impl Default for ServerEvent {
    fn default() -> Self {
        ServerEvent::Lobby(lobby::State::new())
    }
}

fn server_loop(server_data: &mut network::Kjetil<ServerEvent>) {
    let mut mode = ServerEvent::default();
    
    loop {
        match mode {
            ServerEvent::Lobby(_) => {
                
                let mut lobby_data = lobby::State::new();
                println!("lobby data init mode");
                
                while let ServerEvent::Lobby(ref mut t) = mode {
                    lobby::server(server_data.update_and_get_input(), t);
                    server_data.send_game_state(&mode);
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
            ServerEvent::Platform1 => {
                todo!();
            }
            ServerEvent::Hjornefotball => {
                todo!();
            }
        }
        std::thread::sleep(Duration::from_millis(16));
    }
}

fn input_macroquad() -> Option<network::PlayerEvent> {
    // flere keys kan trykkes samtidig
    if let Some(k) = get_last_key_pressed() {
        
        println!("client registered key pressed");
        use PlayerEvent::*;
        use network::Keys::*;
        return Some(match k {
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
        })
    }
    
    // TODO: musen deserialisere ikke ellerno
    // if is_mouse_button_pressed(MouseButton::Left) {
    //     return Some(PlayerEvent::MouseLeft(
    //         mouse_position().0,
    //         mouse_position().1)
    //     )
    // }
    //
    // if is_mouse_button_pressed(MouseButton::Right) {
    //     return Some(PlayerEvent::MouseRight(
    //         mouse_position().0,
    //         mouse_position().1)
    //     )
    // }
    None
}

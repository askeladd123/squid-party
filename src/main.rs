mod lobby;
mod menu;
mod player;
mod platform1;
mod common;
mod hjornefotball;
mod network;

use std::borrow::{Borrow, BorrowMut};
use std::net::{Ipv4Addr, TcpStream};
use std::time::Duration;
use egui_macroquad::draw;
use local_ip_address::local_ip;
use macroquad::prelude::*;
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
                let mut data = menu::main::Data::new();
                
                while matches!(common_data.mode, MenuMode::Main){
                    menu::main::tick(&mut data, &mut common_data);
                    next_frame().await;
                }
            }
            MenuMode::Multiplayer => {
                let mut data = menu::multiplayer::Data::new();
                
                while matches!(common_data.mode, MenuMode::Multiplayer) {
                    menu::multiplayer::tick(&mut data, &mut common_data);
                    next_frame().await;
                }
            },
            MenuMode::SinglePlayer => {
                // når spillet er ferdig går du tilbake til menyen
                common_data.mode = MenuMode::Multiplayer;
                
                // init
                network::start_server("localhost", server_loop).unwrap();
    
                let mut waiter = network::Client::<ServerEvent>::connect_to("localhost");
                let mut client = loop {
                    waiter = match waiter.try_get(){
                        Ok(c) => break c,
                        Err(e) => {
                
                            clear_background(BLACK);
                            draw_text("connecting to host", 100.0, 100.0, 50.0, WHITE);
                
                            e
                        }
                    };
                    next_frame().await;
                };
                
                // TODO: flytte loopen inn i client, så du kan deklarere variabler lettere?
                
                client_loop(&mut client, &mut common_data).await; // er samme som det over
                
            },
            MenuMode::Options => common_data.mode = MenuMode::Main,
            MenuMode::Joining => {
                let mut ip = std::rc::Rc::new(String::new());
                let mut port = std::rc::Rc::new(String::new());
                let mut written = Box::new(false);
                while common_data.mode == MenuMode::Joining {
                    clear_background(BLACK);
                    
                    egui_macroquad::ui(|egui_ctx| {
                        egui_macroquad::egui::Window::new("join host").show(egui_ctx, |ui| {
                            ui.label("type in host ip address: ");
                            ui.text_edit_singleline(std::rc::Rc::get_mut(&mut ip).unwrap());
                            if ui.button("enter").clicked() {
    
                                if ip.parse::<Ipv4Addr>().is_err(){
                                    
                                    *written = false;
                                    panic!("not a ipv4 address this one innit");
                                }
                                
                                // TODO: sjekk om text matcher en ip
                                *written = true;
                            };
                            ui.label("type in port number: ");
                            ui.text_edit_singleline(std::rc::Rc::get_mut(&mut port).unwrap());
                            if ui.button("enter").clicked() {
    
                                if port.parse::<u16>().is_err(){
                                    
                                    *written = false;
                                    panic!("not a port number this one init");
                                }
                                
                            };
                        });
                    });
                    egui_macroquad::draw();
                    
                    if *written {
                        common_data.mode = MenuMode::Joined {
                            host:ip.parse().unwrap(),
                            port:port.parse().unwrap()
                        }
                    }
                    next_frame().await;
                }
            }
            MenuMode::Joined{host, port} => {
                
                let mut waiter = network::Client::<ServerEvent>::connect_to(
                    host.to_string().as_str()
                );
                let mut client = loop {
                    waiter = match waiter.try_get(){
                        Ok(c) => break c,
                        Err(e) => {
                            
                            clear_background(BLACK);
                            draw_text("connecting to host", 100.0, 100.0, 50.0, WHITE);
                            
                            e
                        }
                    };
                    next_frame().await;
                };
                
                // TODO: flytte loopen inn i client, så du kan deklarere variabler lettere?
    
                client_loop(&mut client, &mut common_data).await;
                
            }
            MenuMode::Host => {
                
                let ip = local_ip().unwrap().to_string();
                
                network::start_server(ip.as_str(), server_loop).unwrap();
    
                let mut waiter = network::Client::<ServerEvent>::connect_to(ip.as_str());
                let mut client = loop {
                    waiter = match waiter.try_get(){
                        Ok(c) => break c,
                        Err(e) => {
                
                            clear_background(BLACK);
                            draw_text("connecting to host", 100.0, 100.0, 50.0, WHITE);
                
                            e
                        }
                    };
                    next_frame().await;
                };
                
                client_loop(&mut client, &mut common_data).await;
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

async fn client_loop(client:&mut Client<ServerEvent>, common_data: &mut common::Data){

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
        });
    }
    
    use PlayerEvent::*;
    use network::Keys::*;
    
    if is_key_released(KeyCode::Up) { return Some(Released(Up)); }
    if is_key_released(KeyCode::Down) { return Some(Released(Down)); }
    if is_key_released(KeyCode::Left) { return Some(Released(Left)); }
    if is_key_released(KeyCode::Right) { return Some(Released(Right)); }
    
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

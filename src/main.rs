extern crate core;

// mod lobby;
mod menu;
mod player;
// mod platform1;
mod common;
// mod hjornefotball;
mod network;

use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::Deref;
use std::time::Duration;
use macroquad::prelude::*;
use common::input;
use player::Player;
use crate::common::MenuMode;
use crate::network::{Client, Keys, PlayerEvent, Waiter};

// TODO flytte kode ut i andre filer (så man kan samarbeide)

#[macroquad::main("BetaDev")]
async fn main() {
    
    let mut common_data = common::Data::new(common::files::Data::new().await);
    let mut players: Vec<Player> = vec![
        Player{
            position: physics::Vector2d{x:100.0, y: 100.0},
            speed: physics::Vector2d{x:0.0, y:0.0},
            acceleration: 1.2
        },
        Player{
            position: physics::Vector2d{x:100.0, y: 100.0},
            speed: physics::Vector2d{x:0.0, y:0.0},
            acceleration: 1.0
        },
        Player {
            position: physics::Vector2d { x: 100.0, y: 100.0 },
            speed: physics::Vector2d{ x: 0.0, y: 0.0 },
            acceleration: 0.8
        },
        Player {
            position: physics::Vector2d { x: 100.0, y: 100.0 },
            speed: physics::Vector2d{ x: 0.0, y: 0.0 },
            acceleration: 0.6
        },
        Player {
            position: physics::Vector2d { x: 100.0, y: 100.0 },
            speed: physics::Vector2d{ x: 0.0, y: 0.0 },
            acceleration: 0.4
        },
    
    ];
    
    
    /*
    
    loop
        menu mode
            main
            singleplayer
            multiplayer
                valg
                    host -> ingame
                    join
            join
    
     */
    
    loop {
        match common_data.mode{
    
            MenuMode::Quit => break,
            MenuMode::Main => {
                let mut menu_data = menu::Data::new();
    
                while common_data.mode == MenuMode::Main {
                    menu::tick(&mut menu_data, &mut common_data);
                    next_frame().await;
                }
            }
            MenuMode::SinglePlayer=>{
                // når spillet er ferdig går du tilbake til menyen
                common_data.mode = MenuMode::Main;
                
                // init
                network::start_server("localhost", server_loop);
                
                // loop
                let mut waiter = network::Client::<ServerEvent>::connect("localhost");
                let mut client = loop{
                    // TODO: connecting screen
                    println!("client waiting for connection");
                    
                    waiter = match waiter.try_get(){
                        Ok(t) => break t,
                        Err(s) => s,
                    };
                    
                    next_frame().await;
                };
                
                loop{
                    client.send_input(input_macroquad);
                    
                    match client.get_game_state(){
                        ServerEvent::Lobby => {}
                        ServerEvent::Platform1 => {}
                        ServerEvent::Hjornefotball => {}
                    }
                    
                    next_frame().await;
                }
            }
            MenuMode::Options => {}
            MenuMode::MultiPlayer => {
                while common_data.mode == MenuMode::MultiPlayer{
                    todo!() // send til enten host eller join
                }
            }
            MenuMode::Join => {
                
                let mut ip = std::rc::Rc::new(String::new());
                let mut written = Box::new(false);
                while common_data.mode == MenuMode::Join{
                    clear_background(BLACK);
                    
                    egui_macroquad::ui(|egui_ctx| {
                        egui_macroquad::egui::Window::new("egui ❤ macroquad")
                            .show(egui_ctx, |ui| {
                                ui.label("type in server ip: ");
                                ui.text_edit_singleline(&mut ip.to_string());
                                if ui.button("enter").clicked(){
                                    // TODO: sjekk om text matcher en ip
                                    *written = true;
                                };
                            });
                    });
                    egui_macroquad::draw();
                    
                    if *written{
                        // network::start_client((*ip).clone(), client_loop).await;
                    }
                }
            }
            MenuMode::Host=>{
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

#[derive(serde::Serialize, serde::Deserialize)]
enum ServerEvent{
    Lobby/*(lobby::State)*/,
    Platform1/*(platform1::State)*/,
    Hjornefotball/*(hjornefotball::State)*/,
}

impl Default for ServerEvent{
    fn default() -> Self {
        Self::Lobby
    }
}

fn client_loop(client_data:&mut network::Client<ServerEvent>){
    loop {
        clear_background(BLACK);
    
        println!("nigga client");

        client_data.get_game_state();
        std::thread::sleep(Duration::from_micros(5000));
    }
}

fn server_loop(server_data:&mut network::ServerData<ServerEvent>){
    loop{
        // clear_background(BLACK);
        
        // println!("nigga");
        
        let v = server_data.update_and_get_input();
        for i in v.iter_mut(){
            if i.key_pressed(Keys::Up){
                println!("up");
            }
        }
        
        std::thread::sleep(Duration::from_micros(5000));
    }
}

fn input_macroquad() ->Option<network::PlayerEvent>{
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
        })
    }
    
    if is_mouse_button_pressed(MouseButton::Left) {
        return Some(PlayerEvent::MouseLeft(
            mouse_position().0,
            mouse_position().1)
        )
    }
    
    if is_mouse_button_pressed(MouseButton::Right) {
        return Some(PlayerEvent::MouseRight(
            mouse_position().0,
            mouse_position().1)
        )
    }
    None
}

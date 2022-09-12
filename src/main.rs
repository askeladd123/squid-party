mod lobby;
mod menu;
mod player;
mod platform1;
mod common;
mod hjornefotball;
mod network;

use macroquad::prelude::*;
use common::input;
use player::Player;
use crate::common::MenuMode;
use crate::network::{start_client, start_server};

#[macroquad::main("BetaDev")]
async fn main() {
    
    use physics::*;
    println!("intersection: {}", intersection(
        Shape::AABB(AABB{center:Vector2d{x:0.0, y:0.0}, rx:100.0, ry:100.0}),
        Shape::AABB(AABB{center:Vector2d{x:10.0, y:10.0}, rx:100.0, ry:100.0}),
    ));
    
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
                common_data.mode == MenuMode::Main;
                
                network::start_server();
                network::start_client(
                    network::chain("localhost", ":", network::PORT).as_str(),
                    client_loop
                );
            }
            MenuMode::Options => {}
            MenuMode::MultiPlayer => {
                while common_data.mode == MenuMode::MultiPlayer{
                    todo!() // send til enten host eller join
                }
            }
            MenuMode::Join => {
                
                let ip = std::rc::Rc::new(String::new());
                while common_data.mode == MenuMode::Join{
                    clear_background(BLACK);
                    
                    egui_macroquad::ui(|egui_ctx| {
                        egui_macroquad::egui::Window::new("egui ❤ macroquad")
                            .show(egui_ctx, |ui| {
                                ui.label("type in server ip: ");
                                ui.text_edit_singleline(&mut *ip);
                                if ui.button("enter"){
                                    // TODO: sjekk om text matcher en ip
                                    
                                    common_data.mode == MenuMode::Main;
                                    start_client(
                                        ip.as_str(),
                                        client_loop
                                    );
                                };
                            });
                    });
                    egui_macroquad::draw();
                }
            }
            MenuMode::Host=>{
                common_data.mode == MenuMode::Main;
                
                start_server();
                start_client(
                    network::chain(
                        local_ip_address::local_ip().unwrap().to_string().as_str(),
                        ":",
                        network::PORT
                    ).as_str(),
                    client_loop
                );
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

fn client_loop(client_data:network::ClientData){

}

fn server_loop(){

}
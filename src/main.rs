mod lobby;
mod menu;
mod player;
mod platform1;
mod common;
mod hjornefotball;

use macroquad::prelude::*;
use common::input;
use player::Player;
use crate::common::Mode;

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
    
    loop {
        match common_data.mode{
    
            Mode::Menu=> {
                let mut menu_data = menu::Data::new();
                
                while common_data.mode == Mode::Menu {
                    menu::tick(&mut menu_data, &mut common_data);
                    next_frame().await;
                }
            },
    
            Mode::Lobby=> {
                let mut lobby_data = lobby::Data::new();

                while common_data.mode == Mode::Lobby{
                    lobby::tick(&mut lobby_data, &mut common_data, &mut players);
                    next_frame().await;
                }
            },
    
            Mode::Platform1 => {
                let mut platform1_data = platform1::Data::new();
                
                while common_data.mode == Mode::Platform1 {

                    platform1::tick(&mut platform1_data);
                    next_frame().await;
                }
            },
            Mode::Hjornefotball=> {
                //let mut hjornefotball_data = hjornefotball::Data::new();

                while common_data.mode == Mode::Hjornefotball{
                    hjornefotball::tick();
                    next_frame().await;
                }
            },
    
            Mode::Quit => break,
        }
    }
}

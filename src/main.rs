mod lobby;
mod menu;
mod player;
mod maths;
mod platform1;
mod common;

use macroquad::prelude::*;
use common::input;
use player::Player;
use maths::*;
use crate::common::Mode;

#[macroquad::main("BetaDev")]
async fn main() {
    
    let mut common_data = common::Data::new(common::files::Data::new().await);
    let mut players: Vec<Player> = vec![
        Player{
            position: Vector{x:100.0, y: 100.0},
            speed: Vector{x:0.0, y:0.0},
        }];
    
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
            Mode::Platform2=> {
                let mut lobby_data = lobby::Data::new();

                while common_data.mode == Mode::Platform2{
                    lobby::tick(&mut lobby_data, &mut common_data, &mut players);
                    next_frame().await;
                }
            },
    
            Mode::Quit => break,
        }
    }
}

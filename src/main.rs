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

#[macroquad::main("BetaDev")]
async fn main() {
    
    /*
    Her ligger all data i spillet,
    structs, Vectors, vaiabler.
    Du lager det her, også sender du de
    til funksjonene i loopen under.
     */
    
    let mut common_data = common::Data::new(common::files::Data::new().await);
    let mut menu_data = menu::Data::new();
    // let mut mouse_and_keys = input::MouseAndKeys::default();
    let mut players: Vec<Player> = vec![
        Player{
            position: Vector{x:100.0, y: 100.0},
            speed: Vector{x:0.0, y:0.0},
        }];

    /*
    Denne loopen kjører ca. 60x/s.
    Her kjører man funksjonene.
     */
    loop {
        
        match common_data.mode{
            
            common::Mode::Menu=> menu::tick(&mut menu_data,&mut common_data),
            
            common::Mode::Lobby=> lobby::tick(&mut common_data, &mut players),
            
            common::Mode::Platform1 => platform1::tick(),
            
            common::Mode::Quit => break,
        }
    
        next_frame().await
    }
}

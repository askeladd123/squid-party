use local_ip_address::local_ip;
use macroquad::prelude::*;
use crate::player::Player;
use crate::{common, network};
use crate::network::PlayerInput;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct State{
    pub(crate) players: Vec<Player>,
}

impl State{
    pub fn new()->Self{
    
        use crate::player::*;
        
        State{
            players: vec![
                    Player {
                        position: Vector2d { x: 100.0, y: 100.0 },
                        speed: Vector2d { x: 0.0, y: 0.0 },
                        acceleration: 1.2,
                    },
                    Player {
                        position: Vector2d { x: 100.0, y: 100.0 },
                        speed: Vector2d { x: 0.0, y: 0.0 },
                        acceleration: 1.0,
                    },
                    Player {
                        position: Vector2d { x: 100.0, y: 100.0 },
                        speed: Vector2d { x: 0.0, y: 0.0 },
                        acceleration: 0.8,
                    },
                    Player {
                        position: Vector2d { x: 100.0, y: 100.0 },
                        speed: Vector2d { x: 0.0, y: 0.0 },
                        acceleration: 0.6,
                    },
                    Player {
                        position: Vector2d { x: 100.0, y: 100.0 },
                        speed: Vector2d { x: 0.0, y: 0.0 },
                        acceleration: 0.4,
                    },
                ],
        }
    }
}

pub struct Data{

}

impl Data{
    pub fn new()->Data{
        Data{
        
        }
    }
}

pub fn tick(data: &mut Data, common: &mut common::Data, players: &mut Vec<Player>,){
    // // Mater common og player inn i funksjonen logic
    // logic(common, players);
    // // Mater common.files og players inn i funksjonen logic
    // graphics(&common.files, players);
}

/// Teppet som Sigurd gjemmer seg bak
const TEPPE: f32 = 1000.0;


pub fn server(player_inputs: &mut Vec<PlayerInput>, state: &mut State){
    
    const PLAYER_SPEED:f32 = 1.0;
    
    for (input, player)
    in player_inputs.iter_mut().zip(state.players.iter_mut()){
        
        // // Ser om spilleren sin posisjon er s
        // // tøre enn teppet sitt
        // // Hvis den er det vil common:mode bli endret til platform1
        // if TEPPE < state.players[0].position.x{
        //     // todo change dis åne
        //     // common.mode = common::MenuMode::Platform1;
        //     return;
        // }
        //
        // if 30.0 > state.players[0].position.x{
        //     // todo change dis åne
        //     // common.mode = common::MenuMode::Hjornefotball;
        //     return;
        // }
        //
        // Trykkes OPP knappen vil x fart stoppe og spiller y pos gå TIL OPPOVER i skjermen.
        // x pos STOP
        // y pos -
        if input.key_is_down(network::Keys::Up){
            player.speed.x = 0.0;
            player.speed.y = -PLAYER_SPEED;
        }
        
        // Trykkes NED knappen vil x fart stoppe og spiller y pos gå TIL NEDOVER  i skjermen.
        // x pos STOP
        // y pos +
        else if input.key_is_down(network::Keys::Down){
            player.speed.x = 0.0;
            player.speed.y = PLAYER_SPEED;
        }
    
        // Trykkes VENSTRE knappen vil y fart stoppe og spiller x pos gå TIL VENSTRE i skjermen.
        // x pos -
        // y pos STOP
        else if input.key_is_down(network::Keys::Left){
            player.speed.x = -PLAYER_SPEED;
            player.speed.y = 0.0;
        }
        
        // if is_key_pressed(KeyCode::Left){
        //     state.players[0].speed.x = -PLAYER_SPEED;
        //     state.players[0].speed.y = 0.0;
        // }
    
        // Trykkes HØYRE knappen vil y fart stoppe og spiller x pos gå TIL HØYRE i skjermen.
        // x pos +
        // y pos STOP
        else if input.key_is_down(network::Keys::Right){
            player.speed.x = PLAYER_SPEED;
            player.speed.y = 0.0;
        }
        
        else {
            player.speed.x = 0.0;
            player.speed.y = 0.0;
        }
        
        // if is_key_pressed(KeyCode::Right){
        //     state.players[0].speed.x = PLAYER_SPEED;
        //     state.players[0].speed.y = 0.0;
        // }
    
        player.position.x += player.speed.x;
        player.position.y += player.speed.y;
        // state.players[0].position.x += state.players[0].speed.x;
        // state.players[0].position.y += state.players[0].speed.y;
        // state.players[1].position.x += state.players[0].speed.x *2.2;
        // state.players[1].position.y += state.players[0].speed.y *2.2;
        // state.players[2].position.x += state.players[0].speed.x *1.7;
        // state.players[2].position.y += state.players[0].speed.y *1.7;
        // state.players[3].position.x += state.players[0].speed.x * 1.2;
        // state.players[3].position.y += state.players[0].speed.y * 1.2;
        // state.players[4].position.x += state.players[0].speed.x * 0.6;
        // state.players[4].position.y += state.players[0].speed.y * 0.6;
    }
}

pub fn client(
    files: & common::files::Data,
    // player har posisjon og speed lagret som vektorer.
    state: & State,
){
    clear_background(BLACK);
    
    let pics = [&files.knut, &files.skag, &files.ask, &files.sig];
    
    for (player, pic) in state.players.iter().zip(pics){
    
        // Draw_tecture(Hvordan det så ut, x pos, y pos, farge)
        draw_texture(
            *pic,
            player.position.x,
            player.position.y,
            WHITE,
        );
    }
    // Tegner teppet på skjermen
    draw_rectangle(TEPPE, 0.0, 400.0, 800.0, DARKBLUE);
    
    draw_text(local_ip().unwrap().to_string().as_str(), 24.0, 24.0, 40.0, WHITE);
}
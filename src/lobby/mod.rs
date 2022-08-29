use macroquad::prelude::*;
use crate::player::Player;
use crate::common;

pub struct Data{

}

impl Data{
    pub fn new()->Data{
        Data{
        
        }
    }
}

pub fn tick(data: &mut Data, common: &mut common::Data, players: &mut Vec<Player>,){
    // Mater common og player inn i funksjonen logic
    logic(common, players);
    // Mater common.files og players inn i funksjonen logic
    graphics(&common.files, players);
}

/// Teppet som Sigurd gjemmer seg bak
const TEPPE: f32 = 300.0;


fn logic(common: &mut common::Data, players: &mut Vec<Player>){
    let player = &mut (*players)[common.player_id];

    // Ser om spilleren sin posisjon er støre enn teppet sitt
    // Hvis den er det vil common:mode bli endret til platform1
    if TEPPE < player.position.x{
        common.mode = common::Mode::Platform1;
        return;
    }

    // Trykkes OPP knappen vil x fart stoppe og spiller y pos gå TIL OPPOVER i skjermen.
    // x pos STOP
    // y pos -
    if is_key_pressed(KeyCode::Up){
        player.speed.x = 0.0;
        player.speed.y = -common.settings.player_speed;
    }

    // Trykkes NED knappen vil x fart stoppe og spiller y pos gå TIL NEDOVER  i skjermen.
    // x pos STOP
    // y pos +
    if is_key_pressed(KeyCode::Down){
        player.speed.x = 0.0;
        player.speed.y = common.settings.player_speed;
    }

    // Trykkes VENSTRE knappen vil y fart stoppe og spiller x pos gå TIL VENSTRE i skjermen.
    // x pos -
    // y pos STOP
    if is_key_pressed(KeyCode::Left){
        player.speed.x = -common.settings.player_speed;
        player.speed.y = 0.0;
    }

    // Trykkes HØYRE knappen vil y fart stoppe og spiller x pos gå TIL HØYRE i skjermen.
    // x pos +
    // y pos STOP
    if is_key_pressed(KeyCode::Right){
        player.speed.x = common.settings.player_speed;
        player.speed.y = 0.0;
    }
    
    for player in players{
        // For hver loop vil player.speed legges til i posisjonen og gjøre at spilleren
        // flytter på seg
        player.position.x += player.speed.x;
        player.position.y += player.speed.y;
    }
}

fn graphics(
    files: & common::files::Data,
    // player har posisjon og speed lagret som vektorer.
    players: &mut Vec<Player>,
){
    clear_background(BLACK);
    
    for player in players{
        // Draw_tecture(Hvordan det så ut, x pos, y pos, farge)
        draw_texture(
            files.ask,
            player.position.x,
            player.position.y,
            WHITE,
        );
    }
    // Tegner teppet på skjermen
    draw_rectangle(TEPPE, 0.0, 400.0, 800.0, DARKGRAY);
}
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
const TEPPE: f32 = 1000.0;


fn logic(common: &mut common::Data, players: &mut Vec<Player>){


    // Ser om spilleren sin posisjon er s
    // tøre enn teppet sitt
    // Hvis den er det vil common:mode bli endret til platform1
    if TEPPE < players[0].position.x{
        common.mode = common::MenuMode::Platform1;
        return;
    }

    if 30.0 > players[0].position.x{
        common.mode = common::MenuMode::Hjornefotball;
        return;
    }



    // Trykkes OPP knappen vil x fart stoppe og spiller y pos gå TIL OPPOVER i skjermen.
    // x pos STOP
    // y pos -
    if is_key_pressed(KeyCode::Up){
        players[0].speed.x = 0.0;
        players[0].speed.y = -common.settings.player_speed;
    }

    // Trykkes NED knappen vil x fart stoppe og spiller y pos gå TIL NEDOVER  i skjermen.
    // x pos STOP
    // y pos +
    if is_key_pressed(KeyCode::Down){
        players[0].speed.x = 0.0;
        players[0].speed.y = common.settings.player_speed;
    }

    // Trykkes VENSTRE knappen vil y fart stoppe og spiller x pos gå TIL VENSTRE i skjermen.
    // x pos -
    // y pos STOP
    if is_key_pressed(KeyCode::Left){
       players[0].speed.x = -common.settings.player_speed;
       players[0].speed.y = 0.0;
    }

    // Trykkes HØYRE knappen vil y fart stoppe og spiller x pos gå TIL HØYRE i skjermen.
    // x pos +
    // y pos STOP
    if is_key_pressed(KeyCode::Right){
        players[0].speed.x = common.settings.player_speed;
        players[0].speed.y = 0.0;
    }


    players[0].position.x += players[0].speed.x;
    players[0].position.y += players[0].speed.y;
    players[1].position.x += players[0].speed.x *2.2;
    players[1].position.y += players[0].speed.y *2.2;
    players[2].position.x += players[0].speed.x *1.7;
    players[2].position.y += players[0].speed.y *1.7;
    players[3].position.x += players[0].speed.x * 1.2;
    players[3].position.y += players[0].speed.y * 1.2;
    players[4].position.x += players[0].speed.x * 0.6;
    players[4].position.y += players[0].speed.y * 0.6;



}

fn graphics(
    files: & common::files::Data,
    // player har posisjon og speed lagret som vektorer.
    players: &mut Vec<Player>,
){
    clear_background(BLACK);


        // Draw_tecture(Hvordan det så ut, x pos, y pos, farge)
        draw_texture(

            files.knut,
            players[1].position.x,
            players[1].position.y,
            WHITE,



        );
    draw_texture(

        files.skag,
        players[2].position.x,
        players[2].position.y,
        WHITE,



    );
    draw_texture(

        files.ask,
        players[3].position.x,
        players[3].position.y,
        WHITE,



    );
    draw_texture(

        files.sig,
        players[4].position.x,
        players[4].position.y,
        WHITE,



    );
    // Tegner teppet på skjermen
    draw_rectangle(TEPPE, 0.0, 400.0, 800.0, DARKBLUE);
}
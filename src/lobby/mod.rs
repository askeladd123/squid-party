use macroquad::prelude::*;
use crate::player::Player;
use crate::common;

pub fn tick(common: &mut common::Data, players: &mut Vec<Player>,){
    logic(common, players);
    graphics(&common.files, players);
}

fn logic(
    common: &mut common::Data,
    players: &mut Vec<Player>
){
    if is_key_pressed(KeyCode::Up){
        let speed = &mut (*players)[common.player_id].speed;
        speed.x = 0.0;
        speed.y = -common.settings.player_speed;
    }
    
    if is_key_pressed(KeyCode::Down){
        let speed = &mut (*players)[common.player_id].speed;
        speed.x = 0.0;
        speed.y = common.settings.player_speed;
    }
    
    if is_key_pressed(KeyCode::Left){
        let speed = &mut (*players)[common.player_id].speed;
        speed.x = -common.settings.player_speed;
        speed.y = 0.0;
    }
    
    if is_key_pressed(KeyCode::Right){
        let speed = &mut (*players)[common.player_id].speed;
        speed.x = common.settings.player_speed;
        speed.y = 0.0;
    }
    
    for player in players{
        player.position.x += player.speed.x;
        player.position.y += player.speed.y;
    }
}

fn graphics(
    files: & common::files::Data,
    players: &mut Vec<Player>,
){
    for player in players{
        draw_texture(
            files.sig,
            player.position.x,
            player.position.y,
            WHITE,
        );
    }
}

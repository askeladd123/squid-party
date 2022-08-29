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
    logic(common, players);
    graphics(&common.files, players);
}

const TEPPE: f32 = 300.0;

fn logic(common: &mut common::Data, players: &mut Vec<Player>){
    let player = &mut (*players)[common.player_id];
    
    if TEPPE < player.position.x{
        common.mode = common::Mode::Platform1;
        return;
    }
    
    if is_key_pressed(KeyCode::Up){
        player.speed.x = 0.0;
        player.speed.y = -common.settings.player_speed;
    }
    
    if is_key_pressed(KeyCode::Down){
        player.speed.x = 0.0;
        player.speed.y = common.settings.player_speed;
    }
    
    if is_key_pressed(KeyCode::Left){
        player.speed.x = -common.settings.player_speed;
        player.speed.y = 0.0;
    }
    
    if is_key_pressed(KeyCode::Right){
        player.speed.x = common.settings.player_speed;
        player.speed.y = 0.0;
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
    clear_background(BLACK);
    
    for player in players{
        draw_texture(
            files.ask,
            player.position.x,
            player.position.y,
            WHITE,
        );
    }
    
    draw_rectangle(TEPPE, 0.0, 400.0, 800.0, DARKGRAY);
}
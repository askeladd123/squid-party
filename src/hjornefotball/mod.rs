use macroquad::prelude::*;
use crate::common;
use crate::common::input;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct State{
}

pub struct Data{
    pos_x: f32, // x posisjonen til ballen
    pos_y: f32, // y posisjonen til ballen
    acs: f32,   // Akselerasjon
    angle: f32, // Vinkel ballen blir skutt på
}

impl Data{
    pub fn new()->Data{
        Data{
            pos_x: 300.0,
            pos_y: 300.0,
            acs: 0.0,
            angle: 0.0,
        }
    }
}

pub fn tick(
    data: &mut Data,
    common: &mut common::Data){
    logic();
    graphics(data.pos_x, data.pos_y);
}

fn logic(){

}

fn graphics(pos_x: f32, pos_y: f32){
    clear_background(BLACK);
    draw_circle(100.0, 200.0, 40.0, GREEN);
    draw_circle(300.0, 200.0, 40.0, GREEN);
    draw_circle(200.0, 400.0, 40.0, GREEN);
    draw_hexagon(143.0,52.0,32.0,2.0,false,RED,WHITE);
    let ball_rad = 30.0;
    draw_circle(pos_x, pos_y, ball_rad, DARKBLUE); // fotballen
}
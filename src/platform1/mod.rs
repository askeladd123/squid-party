use macroquad::prelude::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct State{

}

pub struct Data{

}

impl Data{
    pub fn new()->Data{
        Data{
        
        }
    }
}

pub fn tick(data: &mut Data){
    logic();
    graphics();
}

fn logic(){

}

fn graphics(){
    clear_background(BLACK);
    draw_circle(100.0, 200.0, 40.0, GREEN);
    draw_circle(300.0, 200.0, 40.0, GREEN);
    draw_circle(200.0, 400.0, 40.0, GREEN);
    draw_hexagon(143.0,52.0,32.0,2.0,false,RED,WHITE);
}
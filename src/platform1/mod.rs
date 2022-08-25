use macroquad::prelude::*;

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
}
use macroquad::prelude::*;
use crate::common;
use crate::common::input;

pub fn tick(data: &mut Data, common: &mut common::Data){
    logic(&mut data.selected_button, &mut common.mouse_and_keys, &mut common.mode);
    graphics(data.selected_button);
}

fn logic(selected_button: &mut Button, mouse_and_keys: &mut input::MouseAndKeys, mode: &mut common::Mode){
    
    if is_key_pressed(KeyCode::Enter){
        match selected_button {
            Button::Start => *mode = common::Mode::Lobby,
            Button::Quit => *mode = common::Mode::Quit,
            _ => {}
        }
    }
    
    if !mouse_and_keys.up_is_down && is_key_pressed(KeyCode::Up){
        mouse_and_keys.up_is_down = false;
        
        *selected_button = match selected_button {
            Button::Start => Button::Start,
            Button::Options => Button::Start,
            Button::Quit => Button::Options,
        }
    }
    
    if !mouse_and_keys.down_is_down && is_key_pressed(KeyCode::Down){
        mouse_and_keys.down_is_down = false;
        
        *selected_button = match selected_button {
            Button::Start => Button::Options,
            Button::Options => Button::Quit,
            Button::Quit => Button::Quit,
        }
    }
    
}

fn graphics(selected_button: Button){
    clear_background(BLACK);
    
    let w = 100.0;
    let x = screen_width() / 2.0 - w / 2.0;
    
    draw_rectangle(x, 40.0, w, 100.0, match selected_button{
        Button::Start => WHITE,
        _ => GRAY
    });
    draw_rectangle(x, 100.0, w, 100.0, match selected_button{
        Button::Options => WHITE,
        _ => GRAY
    });
    draw_rectangle(x, 240.0, w, 100.0, match selected_button{
        Button::Quit => WHITE,
        _ => GRAY
    });
}

#[derive(Copy, Clone)]
enum Button{
    Start, Options, Quit
}

pub struct Data{
    selected_button: Button,
}

impl Data{
    pub fn new()->Data{
        Data{
            selected_button: Button::Start,
        }
    }
}

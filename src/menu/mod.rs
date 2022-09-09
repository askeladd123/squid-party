use macroquad::prelude::*;
use physics::{AABB, intersection, Shape};
use crate::common;
use crate::common::input;


pub struct Data{
    selected_button: Button,
}

impl Data{
    pub fn new()->Data{
        Data{
            // Forteller hvor den selekterte knappen starter.
            selected_button: Button::Start,
        }
    }
}

pub fn tick(
    data: &mut Data,
    common: &mut common::Data){
    // Gir Logic funksjonen informasjon og knappen den er på, om noen kapper blir trykket og
    // hvilket vindu man er i nå
    logic(&mut data.selected_button, &mut common.mouse_and_keys, &mut common.mode);
    // Gir Graphics funksjonen informasjon om hvilken knapp man holder over
    graphics(data.selected_button);

    let b = AABB{
            center:
            physics::Vector2d{
                x:300.0,
                y:300.5,
            },
            rx:50.1,
            ry:40.1,
        };
    let p = AABB{
        center:
        physics::Vector2d{
            x:300.0,
            y:300.5,
        },
        rx:50.1,
        ry:40.1,
    };
    let c = physics::Circle{
        center:
        physics::Vector2d {
            x: mouse_position().0,
            y: mouse_position().1,
        },
        r:55.0,
    };
    let q = physics::Circle{
        center:
        physics::Vector2d {
            x: 500.0,
            y: 500.0,
        },
        r:55.0,
    };


    draw_rectangle(b.center.x - b.rx, b.center.y - b.ry, b.rx * 2.0, b.ry * 2.0,
    if intersection(Shape::AABB(b),
                    Shape::Point(physics::Vector2d{x:mouse_position().0, y:mouse_position().1})){GREEN} else { BLUE });

    draw_circle(c.center.x, c.center.y, c.r,
                if intersection(Shape::AABB(p), Shape::Circle(c)) {WHITE} else {RED});
    /*
    draw_circle(q.center.x, q.center.y, q.r,
                if intersection(Shape::Circle(q), Shape::Circle(c)) {BLUE} else {GREEN});


     */

}

fn logic(
    selected_button: &mut Button,
    mouse_and_keys: &mut input::MouseAndKeys,
    mode: &mut common::Mode){
    
    if is_key_pressed(KeyCode::Enter){
        match selected_button {
            // Hvis start knappen blir trykket gjør denne linjen at du blir sent til lobbyen
            Button::Start => *mode = common::Mode::Lobby,
            // Hvis Quit knappen er trykket gjør denne linjen at man avslutter spillet
            Button::Quit => *mode = common::Mode::Quit,
            _ => {}
        }
    }

    // Disse to if statmenten gjør at man ikke kan holde ned knappen, når man trykker ned går man
    // - bare en gang ned/opp
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

// Her blir det displayet det grafiske delen av menyen
fn graphics(selected_button: Button){
    clear_background(BLACK);

    // To IKKE mut variabler som skal bestemme høyden / bredden på det grafiske
    let w = 100.0;
    let x = screen_width() / 2.0 - w / 2.0;

    // Under er blir alle knappene i menyen laget, hvis knappen er hvit, er det der man er.
    // Her blir det laget en knapp for å starte spillet
    draw_rectangle(x, 40.0, w, 100.0, match selected_button{
        Button::Start => WHITE,
        _ => GRAY
    });
    // Her blir det laget en knapp for å velge innstillinger
    draw_rectangle(x, 100.0, w, 100.0, match selected_button{
        Button::Options => WHITE,
        _ => GRAY
    });
    // Her blir det laget en knapp for å avsluttet spillet
    draw_rectangle(x, 240.0, w, 100.0, match selected_button{
        Button::Quit => WHITE,
        _ => GRAY
    });
}

#[derive(Copy, Clone)]
// Lager en variabel type, navnet på variabelen sier hvilken "state" menyknappen er i
enum Button{
    Start, Options, Quit
}

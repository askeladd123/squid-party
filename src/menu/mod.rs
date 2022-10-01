
pub mod main{
    
    use crate::common;
    use macroquad::prelude::*;
    
    /// Lager en variabel type, navnet på variabelen sier hvilken "state" menyknappen er i
    #[derive(Copy, Clone)]
    enum Button{
        Singleplayer, Multiplayer, Options, Quit
    }
    
    pub struct Data{
        selected_button: Button,
    }
    
    impl Data{
        pub fn new()->Data{
            Data{
                // Forteller hvor den selekterte knappen starter.
                selected_button: Button::Singleplayer,
            }
        }
    }
    
    pub fn tick(
        data: &mut Data,
        common: &mut common::Data){
        
        // logic
        if is_key_pressed(KeyCode::Enter){
            match data.selected_button {
                // Hvis start knappen blir trykket gjør denne linjen at du blir sent til lobbyen
                Button::Singleplayer => common.mode = common::MenuMode::SinglePlayer,
                Button::Multiplayer => common.mode = common::MenuMode::Multiplayer,
                Button::Options => common.mode = common::MenuMode::Options,

                // Hvis Quit knappen er trykket gjør denne linjen at man avslutter spillet
                Button::Quit => common.mode = common::MenuMode::Quit,
            }
        }
        
        // Disse to if statmenten gjør at man ikke kan holde ned knappen, når man trykker ned går man
        // - bare en gang ned/opp
        if !common.mouse_and_keys.up_is_down && is_key_pressed(KeyCode::Up){
            common.mouse_and_keys.up_is_down = false;
            
            data.selected_button = match data.selected_button {
                Button::Singleplayer => Button::Singleplayer,
                Button::Multiplayer => Button::Singleplayer,
                Button::Options => Button::Multiplayer,
                Button::Quit => Button::Options,
            }
        }
        
        if !common.mouse_and_keys.down_is_down && is_key_pressed(KeyCode::Down){
            common.mouse_and_keys.down_is_down = false;
            
            data.selected_button = match data.selected_button {
                Button::Singleplayer => Button::Multiplayer,
                Button::Multiplayer => Button::Options,
                Button::Options => Button::Quit,
                Button::Quit => Button::Quit,
            }
        }
        
        // graphics
        clear_background(BLACK);
        
        // To IKKE mut variabler som skal bestemme høyden / bredden på det grafiske
        let (w, h) = (200.0, 100.0);
        let x= screen_width() / 2.0 - w / 2.0;
        let font_size = 40.0;
        
        // Under er blir alle knappene i menyen laget, hvis knappen er hvit, er det der man er.
        // Her blir det laget en knapp for å starte spillet
        let y = 40.0;
        draw_rectangle(x, y, w, h, match data.selected_button{
            Button::Singleplayer => WHITE,
            _ => GRAY
        });
        
        draw_text("singleplayer", x, y + h / 2.0, font_size, BLACK);
        
        // Her blir det laget en knapp for å velge innstillinger
        let y = 100.0;
        draw_rectangle(x, y, w, h, match data.selected_button{
            Button::Multiplayer => WHITE,
            _ => GRAY
        });
        
        draw_text("multiplayer", x, y + h / 2.0, font_size, BLACK);
        
        // Her blir det laget en knapp for options
        let y = 240.0;
        draw_rectangle(x, y, w, h, match data.selected_button{
            Button::Options => WHITE,
            _ => GRAY
        });
        
        draw_text("options", x, y + h / 2.0, font_size, BLACK);
    
        // Her blir det laget en knapp for å avsluttet spillet
        let y = 360.0;
        draw_rectangle(x, y, w, h, match data.selected_button{
            Button::Quit => WHITE,
            _ => GRAY
        });
    
        draw_text("quit", x, y + h / 2.0, font_size, BLACK);
    }
}

pub mod multiplayer {
    
    use crate::common;
    use macroquad::prelude::*;
    
    enum Button{
        Join,
        Host,
        Back,
    }
    
    pub struct Data{
        selected_button:Button,
    }
    
    impl Data{
        pub fn new()->Self{
            Data{
                selected_button:Button::Join,
            }
        }
    }
    
    pub fn tick(
        data: &mut Data,
        common: &mut common::Data){
        // logic
        if is_key_pressed(KeyCode::Enter){
            match data.selected_button {
                // Hvis start knappen blir trykket gjør denne linjen at du blir sent til lobbyen
                Button::Join => common.mode = common::MenuMode::Joining,
                // Hvis Quit knappen er trykket gjør denne linjen at man avslutter spillet
                Button::Host => common.mode = common::MenuMode::Host,
                Button::Back => common.mode = common::MenuMode::Main,
            }
        }
    
        // Disse to if statmenten gjør at man ikke kan holde ned knappen, når man trykker ned går man
        // - bare en gang ned/opp
        if !common.mouse_and_keys.up_is_down && is_key_pressed(KeyCode::Up){
            common.mouse_and_keys.up_is_down = false;
        
            data.selected_button = match data.selected_button {
                Button::Join => Button::Join,
                Button::Host => Button::Join,
                Button::Back => Button::Host,
            }
        }
    
        if !common.mouse_and_keys.down_is_down && is_key_pressed(KeyCode::Down){
            common.mouse_and_keys.down_is_down = false;
        
            data.selected_button = match data.selected_button {
                Button::Join => Button::Host,
                Button::Host => Button::Back,
                Button::Back => Button::Back,
            }
        }
    
        // graphics
        clear_background(BLACK);
    
        // To IKKE mut variabler som skal bestemme høyden / bredden på det grafiske
        let (w, h) = (200.0, 100.0);
        let x= screen_width() / 2.0 - w / 2.0;
        let font_size = 40.0;
    
        // Under er blir alle knappene i menyen laget, hvis knappen er hvit, er det der man er.
        // Her blir det laget en knapp for å starte spillet
        let y = 40.0;
        draw_rectangle(x, y, w, h, match data.selected_button{
            Button::Join => WHITE,
            _ => GRAY
        });
    
        draw_text("join", x, y + h / 2.0, font_size, BLACK);
    
        // Her blir det laget en knapp for å velge innstillinger
        let y = 100.0;
        draw_rectangle(x, y, w, h, match data.selected_button{
            Button::Host => WHITE,
            _ => GRAY
        });
    
        draw_text("host", x, y + h / 2.0, font_size, BLACK);
    
        // Her blir det laget en knapp for å avsluttet spillet
        let y = 240.0;
        draw_rectangle(x, y, w, h, match data.selected_button{
            Button::Back => WHITE,
            _ => GRAY
        });
    
        draw_text("back", x, y + h / 2.0, font_size, BLACK);
    }
}
use macroquad::input::{is_key_pressed, is_key_released};
use crate::{common, KeyCode};

pub mod files;
pub mod settings;
pub mod input;

pub struct Data{
    pub mode: MenuMode,
    pub player_id: usize,
    pub settings: settings::Data,
    pub mouse_and_keys: input::MouseAndKeys,
    pub files: files::Data,
    pub pause: bool,
}

impl Data {
    pub fn new(files: files::Data) -> Data {
        Data {
            mode: MenuMode::Main,
            player_id: 0,
            settings: settings::Data {
                player_speed: 1.0,
            },
            mouse_and_keys: input::MouseAndKeys::default(),
            files,
            pause: false,
        }
    }
}

#[derive(PartialEq)]
/// Denne enum forteller hvilket vindu man er i akkurat nå
pub enum MenuMode {
    Main, SinglePlayer, MultiPlayer, Join, Host, Options, Quit,
}

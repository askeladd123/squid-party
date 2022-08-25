use macroquad::input::{is_key_pressed, is_key_released};
use crate::{common, KeyCode};

pub mod files;
pub mod settings;
pub mod input;

pub struct Data{
    pub mode: Mode,
    pub player_id: usize,
    pub settings: settings::Data,
    pub mouse_and_keys: input::MouseAndKeys,
    pub files: files::Data,
    pub pause: bool,
}

impl Data {
    pub fn new(files: files::Data) -> Data {
        Data {
            mode: Mode::Menu,
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
pub enum Mode {
    Menu, Lobby, Platform1, Quit
}

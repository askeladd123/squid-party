pub mod files;
pub mod settings;
pub mod input;

pub struct Data{
    pub mode: Mode,
    pub player_id: usize,
    pub settings: settings::Data,
    pub mouse_and_keys: input::MouseAndKeys,
    pub files: files::Data,
}

impl Data{
    pub fn new(files: files::Data) -> Data{
        Data{
            mode: Mode::Menu,
            player_id: 0,
            settings: settings::Data {
                player_speed: 0.1,
            },
            mouse_and_keys: input::MouseAndKeys::default(),
            files,
        }
    }
}

pub enum Mode {
    Menu, Lobby, Platform1, Quit
}

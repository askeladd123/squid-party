use macroquad::prelude::*;

pub struct Data{
    pub sig: Texture2D,
    pub skag: Texture2D,
    pub knut: Texture2D,
    pub ask: Texture2D,
    }

impl Data {
    pub async fn new()->Data{
        
        Data{
            sig: load_texture("res/sig.png").await.unwrap(),
            skag: load_texture("res/skagemoe.png").await.unwrap(),
            knut: load_texture("res/knutelute.png").await.unwrap(),
            ask: load_texture("res/ask.png").await.unwrap()



        }
    }
}
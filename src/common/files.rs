use macroquad::prelude::*;

pub struct Data{
    pub sig: Texture2D,
}

impl Data {
    pub async fn new()->Data{
        
        Data{
            sig: load_texture("res/sig.png").await.unwrap(),
        }
    }
}
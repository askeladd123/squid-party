use macroquad::prelude::*;

#[macroquad::main("BetaDev")]
async fn main() {
    loop {
        clear_background(BLACK);
    
        next_frame().await
    }
}
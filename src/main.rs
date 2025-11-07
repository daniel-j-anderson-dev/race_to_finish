use macroquad::prelude::*;

#[macroquad::main("race to the finish")]
async fn main() {
    loop {
        clear_background(WHITE);
        next_frame().await;
    }
}
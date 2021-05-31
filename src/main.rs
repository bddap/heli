extern crate core;

use macroquad::prelude::*;

mod constants;
mod system;

#[macroquad::main("Heli")]
async fn main() {
    let mut heli = system::Heli::new();

    while !heli.should_quit() {
        heli.update();
        heli.draw();
        next_frame().await;
    }
}

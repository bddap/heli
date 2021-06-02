extern crate core;

use macroquad::prelude::*;

mod constants;
mod system;
mod util;

#[macroquad::main(window_conf)]
async fn main() {
    let mut heli = system::Heli::new();

    while !heli.should_quit() {
        heli.update();
        heli.draw();
        heli.ui();
        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Heli".to_owned(),
        fullscreen: true,
        high_dpi: false,
        ..Default::default()
    }
}

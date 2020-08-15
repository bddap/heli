mod mesh;
mod system;
mod world;

use ggez::event::{self};
use ggez::ContextBuilder;

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");
    let mut my_game = world::World::new(&mut ctx);
    event::run(&mut ctx, &mut event_loop, &mut my_game).unwrap();
}

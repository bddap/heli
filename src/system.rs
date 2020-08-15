use super::world::World;
use ggez::{
    graphics::{self, DrawParam},
    Context, GameResult,
};

// the one system that calls all others
pub fn tick(worl: &mut World) {
    vel(worl);
    gravity(worl);
}

pub fn draw(worl: &mut World, ctx: &mut Context) -> GameResult<()> {
    for (pos, draw) in worl.pos.iter().zip(worl.draw.iter()) {
        match (pos, draw) {
            (Some(pos), Some(mesh)) => {
                graphics::draw(ctx, mesh, DrawParam::new().dest([pos.0, pos.1]))?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn vel(worl: &mut World) {
    for (pos, vel) in worl.pos.iter_mut().zip(worl.vel.iter()) {
        match (pos, vel) {
            (Some(pos), Some(vel)) => {
                pos.0 += vel.0;
                pos.1 += vel.1;
            }
            _ => {}
        }
    }
}

fn gravity(worl: &mut World) {
    for vel in worl.vel.iter_mut() {
        match vel {
            Some(vel) => {
                vel.0 += World::GRAVITY;
            }
            _ => {}
        }
    }
}

use super::world::World;
use ggez::{
    graphics::{self, DrawParam},
    Context, GameResult,
};

// the one system that calls all others
pub fn tick(worl: &mut World) {
    vel(worl);
    gravity(worl);
    bounce_floor(worl);
}

pub fn draw(worl: &mut World, ctx: &mut Context) -> GameResult<()> {
    for ((pos, draw), rot) in worl.pos.iter().zip(worl.draw.iter()).zip(worl.rot.iter()) {
        match (pos, draw) {
            (Some(pos), Some(mesh)) => {
                let dp = DrawParam::new().dest([pos.0, pos.1]);
                let dp = match rot {
                    Some(rot) => dp.rotation(*rot),
                    None => dp,
                };
                graphics::draw(ctx, mesh, dp)?;
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
    for ((vel, grav), pos) in worl
        .vel
        .iter_mut()
        .zip(worl.grav.iter())
        .zip(worl.pos.iter())
    {
        match (vel, grav, pos) {
            (Some(vel), Some(_g), Some(pos)) if pos.1 > 0.0 => {
                vel.1 += World::GRAVITY;
            }
            (Some(vel), Some(_g), None) => {
                vel.1 += World::GRAVITY;
            }
            _ => {}
        }
    }
}

/// If entity is underground and velocity is downward, reverse velocity
fn bounce_floor(worl: &mut World) {
    for (vel, pos) in worl.vel.iter_mut().zip(worl.pos.iter()) {
        match (vel, pos) {
            (Some(vel), Some(pos)) => {
                if pos.1 < 0.0 && vel.1 < 0.0 {
                    vel.1 = -vel.1;
                }
            }
            _ => {}
        }
    }
}

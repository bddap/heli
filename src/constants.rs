// pub const TICKS_PER_SECOND: usize = 128;
use bevy::math::{Quat, Vec3};
use bevy::transform::components::Transform;

pub const GRAVITY: f32 = -9.8;
pub const CAMERA_SETUP: Transform = Transform {
    translation: Vec3 {
        x: 0.0,
        y: 20.0,
        z: 0.0,
    },
    rotation: Quat::identity(),
    scale: Vec3 {
        x: 0.1,
        y: 0.1,
        z: 0.1,
    },
};
/// in radians per second
pub const ROTATIONAL_ACCELERATION: f32 = 10.0;
pub const ROTATIONAL_DRAG_COEFFICIENT: f32 = 2.0;
pub const DRAG_COEFFICIENT: f32 = 0.1;
pub const BOOST_POWER: f32 = 18.0;

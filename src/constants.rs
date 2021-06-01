pub const GRAVITY: f32 = -9.8;

/// radians per second per second
pub const ROTATIONAL_ACCELERATION: f32 = 10.0;

pub const ROTATIONAL_DRAG_COEFFICIENT: f32 = 1.5;

pub const DRAG_COEFFICIENT: f32 = 0.05;

/// meters per second per second
pub const BOOST_POWER: f32 = 30.0;

/// meters width, height
pub const PLAYER_SIZE: f32 = 10.0;

/// meters
pub const WORLD_HEIGHT: f32 = 500.0;

pub const PLAYER_WIREFRAME: &[(f32, f32)] = &[
    (-0.5 * PLAYER_SIZE, -0.5 * PLAYER_SIZE),
    (-0.1 * PLAYER_SIZE, 0.5 * PLAYER_SIZE),
    (0.1 * PLAYER_SIZE, 0.5 * PLAYER_SIZE),
    (0.5 * PLAYER_SIZE, -0.5 * PLAYER_SIZE),
    (0.0 * PLAYER_SIZE, 0.0 * PLAYER_SIZE),
    (-0.5 * PLAYER_SIZE, -0.5 * PLAYER_SIZE),
];

pub const BOUNDS_WIREFRAME: &[(f32, f32)] = &[
    (-1. * WORLD_HEIGHT / 2., -1. * WORLD_HEIGHT / 2.),
    (-1. * WORLD_HEIGHT / 2., 1.0 * WORLD_HEIGHT / 2.),
    (1.0 * WORLD_HEIGHT / 2., 1.0 * WORLD_HEIGHT / 2.),
    (1.0 * WORLD_HEIGHT / 2., -1. * WORLD_HEIGHT / 2.),
    (-1. * WORLD_HEIGHT / 2., -1. * WORLD_HEIGHT / 2.),
];

/// how hard the ship should try to point upward when boosting up
/// this value is multiplied by rotational error to determine a
/// rotational force
pub const AUTO_UP_POWER: f32 = 2.0;

/// what percentage of velocity is lost on collision
pub const COLLISION_ENERGY_LOSS: f32 = 0.1;

#[test]
fn cel() {
    assert!(COLLISION_ENERGY_LOSS >= 0.0);
    assert!(COLLISION_ENERGY_LOSS <= 1.0);
}

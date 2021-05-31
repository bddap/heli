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
pub const WORLD_WIDTH: f32 = 500.0;

pub const WORLD_ASPECT_RATIO: f32 = 1.3333;

pub const WORLD_HEIGHT: f32 = WORLD_WIDTH / WORLD_ASPECT_RATIO;

pub const PLAYER_WIREFRAME: &[(f32, f32)] = &[
    (-0.5, -0.5),
    (-0.1, 0.5),
    (0.1, 0.5),
    (0.5, -0.5),
    (0.0, 0.0),
];

/// how hard the ship should try to point upward when boosting up
/// this value is multiplied by rotational error to determine a
/// rotational force
pub const AUTO_UP_POWER: f32 = 2.0;

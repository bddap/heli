pub const GRAVITY: f32 = -9.8;

/// radians per second per second
pub const ROTATIONAL_ACCELERATION: f32 = 10.0;

pub const ROTATIONAL_DRAG_COEFFICIENT: f32 = 2.0;

pub const DRAG_COEFFICIENT: f32 = 0.1;

/// meters per second per second
pub const BOOST_POWER: f32 = 18.0;

/// meters width, height
pub const PLAYER_SIZE: f32 = 10.0;

/// meters
pub const WORLD_WIDTH: f32 = 500.0;

pub const PLAYER_WIREFRAME: &[(f32, f32)] = &[
    (-0.5, -0.5),
    (-0.1, 0.5),
    (0.1, 0.5),
    (0.5, -0.5),
    (0.0, 0.0),
];

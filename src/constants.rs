use crate::util::color;
use macroquad::color::Color;
use macroquad::ui::{hash, Ui};

#[derive(Debug, Clone)]
pub struct Settings {
    /// meters per second per second
    pub gravity: f32,
    /// radians per second per second
    pub rotational_acceleration: f32,
    pub rotational_drag_coefficient: f32,
    pub drag_coefficient: f32,
    /// meters per second per second
    pub boost_power: f32,
    /// how hard the ship should try to point upward when boosting up
    /// this value is multiplied by rotational error to determine a
    /// rotational force
    pub auto_up_power: f32,
    /// what percentage of velocity is lost on collision
    pub collision_energy_loss: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            gravity: -9.8,
            rotational_acceleration: 10.0,
            rotational_drag_coefficient: 1.5,
            drag_coefficient: 0.05,
            boost_power: 30.0,
            auto_up_power: 2.0,
            collision_energy_loss: 0.1,
        }
    }
}

impl Settings {
    pub fn ui(&mut self, ui: &mut Ui) {
        let def = Self::default();
        let range = |radius, default| (default - radius)..(default + radius);
        ui.slider(
            hash!(),
            "gravity",
            range(def.gravity, def.gravity),
            &mut self.gravity,
        );
        ui.slider(
            hash!(),
            "rotational_acceleration",
            range(def.rotational_acceleration, def.rotational_acceleration),
            &mut self.rotational_acceleration,
        );
        ui.slider(
            hash!(),
            "rotational_drag_coefficient",
            range(
                def.rotational_drag_coefficient,
                def.rotational_drag_coefficient,
            ),
            &mut self.rotational_drag_coefficient,
        );
        ui.slider(
            hash!(),
            "drag_coefficient",
            range(def.drag_coefficient, def.drag_coefficient),
            &mut self.drag_coefficient,
        );
        ui.slider(
            hash!(),
            "boost_power",
            range(def.boost_power, def.boost_power),
            &mut self.boost_power,
        );
        ui.slider(
            hash!(),
            "auto_up_power",
            range(def.auto_up_power, def.auto_up_power),
            &mut self.auto_up_power,
        );
        ui.slider(
            hash!(),
            "collision_energy_loss",
            range(def.collision_energy_loss, def.collision_energy_loss),
            &mut self.collision_energy_loss,
        );
    }
}

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

pub fn color_pallet() -> [Color; 10] {
    [
        color(0x58, 0x2f, 0x0e),
        color(0x7f, 0x4f, 0x24),
        color(0x93, 0x66, 0x39),
        color(0xa6, 0x8a, 0x64),
        color(0xb6, 0xad, 0x90),
        color(0xc2, 0xc5, 0xaa),
        color(0xa4, 0xac, 0x86),
        color(0x65, 0x6d, 0x4a),
        color(0x41, 0x48, 0x33),
        color(0x33, 0x3d, 0x29),
    ]
}

#[test]
fn cel() {
    assert!(Settings::default().collision_energy_loss >= 0.0);
    assert!(Settings::default().collision_energy_loss <= 1.0);
}

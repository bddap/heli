use crate::constants::GRAVITY;
use bevy::prelude::*;
use core::fmt::Debug;

/// A plugin that registers all custom systems for this game.
pub struct Heli;

impl Plugin for Heli {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(set_boost.system())
            .add_system(apply_boost.system())
            .add_system(apply_vel.system())
            .add_system(bounce_floor.system())
            .add_system(gravity.system())
            .add_system(rotation_input.system())
            .add_system(set_transform.system())
            .add_system(dbug.system())
            .add_system(exit.system())
            .add_system(rot_vel.system())
            .add_system(show_boost.system())
            .add_system(rotation_drag.system())
            .add_system(drag.system());
    }
}

#[derive(Debug)]
pub struct Controls;
#[derive(Debug)]
pub struct Rot(pub f32);
#[derive(Debug)]
pub struct RotVel(pub f32);
#[derive(Debug)]
pub struct Vel(pub Vec2);
#[derive(Debug)]
pub struct Pos(pub Vec2);
#[derive(Debug)]
pub struct Grav;
#[derive(Debug)]
pub struct Drag;
#[derive(Debug)]
pub struct Boost(pub f32);

fn apply_vel(time: Res<Time>, mut query: Query<(&Vel, &mut Pos)>) {
    let delta = time.delta_seconds();
    for (vel, mut pos) in query.iter_mut() {
        pos.0 += vel.0 * delta;
    }
}

fn rot_vel(time: Res<Time>, mut query: Query<(&RotVel, &mut Rot)>) {
    let delta = time.delta_seconds();
    for (rot_vel, mut rot) in query.iter_mut() {
        rot.0 += rot_vel.0 * delta;
    }
}

// if item has gone through the floor, bounce
// this should be run after vel.system()
fn bounce_floor(mut query: Query<(&mut Vel, &mut Pos)>) {
    for (mut vel, mut pos) in query.iter_mut() {
        if pos.0.y <= 0.0 {
            pos.0.y = -pos.0.y;
            vel.0.y = vel.0.y.abs();
            // arrest some horizontal velocity
            vel.0.x /= 2.0;
        }
    }
}

fn gravity(time: Res<Time>, mut query: Query<(&mut Vel, &Grav)>) {
    let delta = time.delta_seconds();
    for (mut vel, Grav) in query.iter_mut() {
        vel.0.y += GRAVITY * delta;
    }
}

fn rotation_input(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Controls, &mut RotVel)>,
) {
    let delta = time.delta_seconds() * crate::constants::ROTATIONAL_ACCELERATION;
    for (Controls, mut rot) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            rot.0 += delta;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            rot.0 -= delta;
        }
    }
}

/// Not physically based. This is incorrect but feels nice as long as delta_t stays small.
fn rotation_drag(time: Res<Time>, mut query: Query<(&mut RotVel, &Drag)>) {
    let delta = time.delta_seconds() * crate::constants::ROTATIONAL_DRAG_COEFFICIENT;
    for (mut rvel, Drag) in query.iter_mut() {
        rvel.0 -= rvel.0 * delta;
    }
}

/// This is incorrect but ok.
fn drag(time: Res<Time>, mut query: Query<(&mut Vel, &Drag)>) {
    let delta = time.delta_seconds() * crate::constants::DRAG_COEFFICIENT;
    for (mut vel, Drag) in query.iter_mut() {
        let a = vel.0 * delta;
        vel.0 -= a;
    }
}

fn dbug(query: Query<&Rot>) {
    for _t in query.iter() {}
}

fn set_transform(mut query: Query<(&Pos, &Rot, &mut Transform)>) {
    for (pos, rot, mut trans) in query.iter_mut() {
        *trans = Transform {
            rotation: Quat::from_rotation_z(rot.0),
            translation: pos.0.extend(0.0),
            scale: Vec3::splat(1.0),
        };
    }
}

fn exit(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) || keyboard_input.pressed(KeyCode::Q) {
        app_exit_events.send(bevy::app::AppExit);
    }
}

/// Set force vector of boost depending on which keys are held.
fn set_boost(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Boost, &Controls)>) {
    for (mut boost, Controls) in query.iter_mut() {
        boost.0 = 0.0;
        if keyboard_input.pressed(KeyCode::Up) {
            boost.0 += crate::constants::BOOST_POWER;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            boost.0 -= crate::constants::BOOST_POWER;
        }
    }
}

/// Modify velocity according to boost.
fn apply_boost(time: Res<Time>, mut query: Query<(&Boost, &mut Vel, &Rot)>) {
    let delta = time.delta_seconds();
    for (boost, mut vel, rot) in query.iter_mut() {
        let rot = Quat::from_rotation_z(rot.0);
        vel.0 += rot.mul_vec3(Vec3::unit_y() * boost.0 * delta).truncate();
    }
}

/// TODO: if boosting, show some visual feedback
fn show_boost() {}

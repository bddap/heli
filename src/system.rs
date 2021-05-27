use crate::constants;
use crate::constants::GRAVITY;
use core::fmt::Debug;
use macroquad::prelude::*;

pub struct Heli {
    world: hecs::World,
}

impl Heli {
    pub fn new() -> Self {
        let mut world = hecs::World::new();

        let player = (
            Controls,
            Rot(0.),
            RotVel(0.),
            Vel((0., 0.).into()),
            Pos((constants::WORLD_WIDTH / 2., constants::WORLD_WIDTH / 4.).into()),
            Grav,
            Drag,
            Boost(0.0),
            DARKBROWN,
        );
        world.spawn(player);

        Self { world }
    }

    pub fn update(&mut self) {
        self.controls();
        self.newtonian();
        self.collision();
    }

    pub fn draw(&self) {
        clear_background(LIGHTGRAY);
        for (_, (c, p, r)) in self.world.query::<(&Color, &Pos, &Rot)>().iter() {
            let q = r.quat();
            draw_wireframe(
                constants::PLAYER_WIREFRAME,
                p.0,
                q,
                constants::PLAYER_SIZE,
                *c,
            );
        }
    }

    pub fn should_quit(&self) -> bool {
        self.world.query::<(&Quit,)>().into_iter().next().is_some()
    }
}

impl Heli {
    fn controls(&mut self) {
        let delta_t = get_frame_time();

        // boost
        let mut boost = 0.0;
        if is_key_down(KeyCode::Up) | is_key_down(KeyCode::W) {
            boost += crate::constants::BOOST_POWER;
        }
        if is_key_down(KeyCode::Down) | is_key_down(KeyCode::S) {
            boost -= crate::constants::BOOST_POWER;
        }
        for (_id, (_controls, b)) in self.world.query_mut::<(&Controls, &mut Boost)>() {
            b.0 = boost;
        }

        // rotation accel
        let mut rot_accel = 0.0;
        if is_key_down(KeyCode::Left) | is_key_down(KeyCode::A) {
            rot_accel += crate::constants::ROTATIONAL_ACCELERATION;
        }
        if is_key_down(KeyCode::Right) | is_key_down(KeyCode::D) {
            rot_accel -= crate::constants::ROTATIONAL_ACCELERATION;
        }
        rot_accel *= delta_t;
        for (_id, (_controls, rv)) in self.world.query_mut::<(&Controls, &mut RotVel)>() {
            rv.0 += rot_accel;
        }

        // quit
        if is_key_down(KeyCode::Escape) || is_key_down(KeyCode::Q) {
            self.world.spawn((Quit,));
        }
    }

    fn newtonian(&mut self) {
        let delta_t = get_frame_time();

        // apply gravity to velocity
        for (_id, (Grav, v)) in self.world.query_mut::<(&Grav, &mut Vel)>() {
            v.0.y += GRAVITY * delta_t;
        }

        // apply boost to velocity
        for (_id, (b, r, v)) in self.world.query_mut::<(&Boost, &Rot, &mut Vel)>() {
            v.0 += r.quat().mul_vec3(Vec3::Y * b.0 * delta_t).truncate();
        }

        // apply velocity to position
        for (_id, (v, p)) in self.world.query_mut::<(&Vel, &mut Pos)>() {
            p.0 += v.0 * delta_t;
        }

        // apply rotational velocity to rotation
        for (_id, (rv, r)) in self.world.query_mut::<(&RotVel, &mut Rot)>() {
            r.0 += rv.0 * delta_t;
        }
    }

    fn collision(&mut self) {
        let delta_t = get_frame_time();

        // if item has gone through the floor, bounce
        // this should be run after vel.system()
        for (_i, (v, p)) in self.world.query_mut::<(&mut Vel, &mut Pos)>() {
            if p.0.y <= 0.0 {
                p.0.y = -p.0.y;
                v.0.y = v.0.y.abs();
                // arrest some horizontal velocity
                v.0.x /= 2.0;
            }
        }

        // collosion with air, also known as drag
        let drag_mult = delta_t * constants::DRAG_COEFFICIENT;
        debug_assert!(drag_mult < 1.0);
        for (_i, (v, Drag)) in self.world.query_mut::<(&mut Vel, &Drag)>() {
            v.0 -= v.0 * drag_mult;
        }

        // rotational drag
        let rdrag_mult = delta_t * constants::ROTATIONAL_DRAG_COEFFICIENT;
        debug_assert!(rdrag_mult < 1.0);
        for (_i, (rv, Drag)) in self.world.query_mut::<(&mut RotVel, &Drag)>() {
            rv.0 -= rv.0 * rdrag_mult;
        }
    }
}

#[derive(Debug)]
pub struct Controls;

#[derive(Debug)]
pub struct Rot(pub f32);

impl Rot {
    fn quat(&self) -> Quat {
        Quat::from_rotation_z(self.0)
    }
}

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

#[derive(Debug)]
pub struct Quit;

fn draw_wireframe(
    wireframe: &[(f32, f32)],
    position: Vec2,
    rotation: Quat,
    scale: f32,
    color: Color,
) {
    debug_assert!(!wireframe.is_empty());
    let sw = screen_width();
    let screenscale = sw / constants::WORLD_WIDTH;
    let sh = screen_height();

    let to_screen = |point: (f32, f32)| {
        let mut p: Vec2 = point.into();
        p *= scale;
        p = rotation.mul_vec3(p.extend(0.0)).truncate();
        p += position;
        p *= screenscale;
        p.y = sh - p.y;
        p
    };

    let screen_coords = wireframe.into_iter().cloned().map(to_screen);
    let shifted = screen_coords.clone().cycle().skip(1).take(wireframe.len());
    for (a, b) in screen_coords.zip(shifted) {
        draw_line(a.x, a.y, b.x, b.y, 2., color);
    }
}

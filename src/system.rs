use crate::constants::*;
use core::f32::consts::{PI, TAU};
use core::fmt::Debug;
use macroquad::prelude::*;
use parry2d::math::Isometry;
use parry2d::math::Vector;
use parry2d::query::time_of_impact;
use parry2d::query::TOIStatus;
use parry2d::query::TOI;
use parry2d::shape::Ball;
use parry2d::shape::Cuboid;
use parry2d::shape::Shape;

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
            Pos((WORLD_WIDTH / 2., WORLD_HEIGHT / 2.).into()),
            Grav,
            Drag,
            Boost(0.0),
            DARKBROWN,
            Collides(Box::new(Ball::new(PLAYER_SIZE / 2.0))),
            Wireframe(PLAYER_WIREFRAME),
        );
        world.spawn(player);

        let player = (
            Controls,
            Rot(0.),
            RotVel(0.),
            Vel((0., 0.).into()),
            Pos((WORLD_WIDTH / 3., WORLD_HEIGHT / 3.).into()),
            Grav,
            Drag,
            Boost(0.0),
            MAROON,
            Collides(Box::new(Ball::new(PLAYER_SIZE / 2.0))),
            Wireframe(PLAYER_WIREFRAME),
        );
        world.spawn(player);

        let walls = [
            (0.0, 0.0, WORLD_WIDTH, 0.0),
            (0.0, 0.0, 0.0, WORLD_HEIGHT),
            (WORLD_WIDTH, 0.0, 0.0, WORLD_HEIGHT),
            (0.0, WORLD_HEIGHT, WORLD_WIDTH, 0.0),
        ];
        for (x, y, w, h) in &walls {
            world.spawn((
                Pos((*x, *y).into()),
                Vel((0., 0.).into()),
                Collides(Box::new(Cuboid::new(Vector::new(*w, *h)))),
            ));
        }

        Self { world }
    }

    pub fn update(&mut self) {
        self.controls();
        self.collision();
        self.newtonian();
    }

    pub fn draw(&self) {
        clear_background(LIGHTGRAY);
        for (_, (c, p, r, w)) in self
            .world
            .query::<(&Color, &Pos, &Rot, &Wireframe)>()
            .iter()
        {
            let q = r.quat();
            draw_wireframe(w.0, p.0, q, PLAYER_SIZE, *c);
        }

        draw_text(&format!("fps: {}", get_fps()), 10.0, 30.0, 30.0, WHITE);
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
            boost += BOOST_POWER;
        }
        if is_key_down(KeyCode::Down) | is_key_down(KeyCode::S) {
            boost -= BOOST_POWER;
        }
        for (_id, (_controls, b)) in self.world.query_mut::<(&Controls, &mut Boost)>() {
            b.0 = boost;
        }

        // rotation accel
        let mut rot_accel = 0.0;
        if is_key_down(KeyCode::Left) | is_key_down(KeyCode::A) {
            rot_accel += ROTATIONAL_ACCELERATION;
        }
        if is_key_down(KeyCode::Right) | is_key_down(KeyCode::D) {
            rot_accel -= ROTATIONAL_ACCELERATION;
        }
        rot_accel *= delta_t;
        for (_id, (_controls, rv)) in self.world.query_mut::<(&Controls, &mut RotVel)>() {
            rv.0 += rot_accel;
        }

        // when boosting up, rotation should tend upwards, it feels better that way
        for (_id, (_controls, rv, r, b)) in self
            .world
            .query_mut::<(&Controls, &mut RotVel, &Rot, &Boost)>()
        {
            if b.0 >= 0.1 {
                let r = (r.0 + PI).rem_euclid(TAU) - PI;
                debug_assert!(r >= -PI - 0.0001);
                debug_assert!(r <= PI + 0.0001);
                let rotvel_delta = -r * AUTO_UP_POWER * delta_t;
                rv.0 += rotvel_delta;
            }
        }

        // quit
        if is_key_down(KeyCode::Escape) || is_key_down(KeyCode::Q) {
            self.world.spawn((Quit,));
        }
    }

    fn collision(&mut self) {
        let delta_t = get_frame_time();

        let mut collisions: Vec<_> = Vec::new();
        for (ia, (va, pa, ca)) in self.world.query::<(&Vel, &Pos, &Collides)>().iter() {
            for (ib, (vb, pb, cb)) in self.world.query::<(&Vel, &Pos, &Collides)>().iter() {
                if ia == ib {
                    continue;
                }
                let impact = time_of_impact(
                    &Isometry::translation(pa.0.x, pa.0.y),
                    &Vector::new(va.0.x, va.0.y),
                    &*ca.0,
                    &Isometry::translation(pb.0.x, pb.0.y),
                    &Vector::new(vb.0.x, vb.0.y),
                    &*cb.0,
                    delta_t,
                )
                .unwrap();
                match impact {
                    None => {}
                    Some(TOI {
                        toi,
                        witness1: _,
                        witness2: _,
                        normal1,
                        normal2,
                        status: TOIStatus::Converged,
                    }) => {
                        collisions.push((ia, ib, toi, normal1, normal2));
                    }
                    Some(TOI { .. }) => {}
                }
            }
        }
        collisions.sort_by(|a, b| (a.0, a.1, a.2).partial_cmp(&(b.0, b.1, b.2)).unwrap());
        collisions.dedup_by(|a, b| (a.0, a.1) == (b.0, b.1));
        for (ia, _ib, toi, normal1, _normal2) in collisions {
            let (vel, pos) = self
                .world
                .query_one_mut::<(&mut Vel, &mut Pos)>(ia)
                .unwrap();
            let v: &mut Vec2 = &mut vel.0;
            let p: &mut Vec2 = &mut pos.0;
            let n: Vec2 = (normal1.x, normal1.y).into();

            // reflect velocity according to normal
            // https://www.youtube.com/watch?v=naaeH1qbjdQ
            let newvel = *v - v.dot(n) * n * 2.0;

            // position is moved into the collision such that the next time velocity is applied
            // position will be outside of the collision
            // this makes for a fully elastic collision
            *p += (*v - newvel) * toi;

            *v = newvel;
        }

        // collision with air, also known as drag
        let drag_mult = delta_t * DRAG_COEFFICIENT;
        debug_assert!(drag_mult < 1.0);
        for (_i, (v, Drag)) in self.world.query_mut::<(&mut Vel, &Drag)>() {
            v.0 -= v.0 * drag_mult;
        }

        // rotational drag
        let rdrag_mult = delta_t * ROTATIONAL_DRAG_COEFFICIENT;
        debug_assert!(rdrag_mult < 1.0);
        for (_i, (rv, Drag)) in self.world.query_mut::<(&mut RotVel, &Drag)>() {
            rv.0 -= rv.0 * rdrag_mult;
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

pub struct Collides(Box<dyn Shape>);

#[derive(Debug)]
pub struct Wireframe(&'static [(f32, f32)]);

fn draw_wireframe(
    wireframe: &[(f32, f32)],
    position: Vec2,
    rotation: Quat,
    scale: f32,
    color: Color,
) {
    debug_assert!(!wireframe.is_empty());
    let sw = screen_width();
    let screenscale = sw / WORLD_WIDTH;
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

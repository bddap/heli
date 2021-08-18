use crate::constants::*;
use crate::mortal::Mortal;
use crate::spawner::firetrail;
use crate::spawner::BoostToots;
use crate::util::wireframe_to_polyline;
use alloc::sync::Arc;
use core::f32::consts::{PI, TAU};
use core::fmt::Debug;
use hecs::Entity;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets::Window, Ui};
use parry2d::math::{Isometry, Vector};
use parry2d::query::{time_of_impact, TOIStatus, TOI};
use parry2d::shape::Shape;

pub struct Heli {
    world: hecs::World,
}

impl Heli {
    pub fn new() -> Self {
        let mut world = hecs::World::new();

        let camera = (Camera2D::default(),);
        world.spawn(camera);

        // player 1
        world.spawn((
            Controls {
                up: KeyCode::Up,
                down: KeyCode::Down,
                left: KeyCode::Left,
                right: KeyCode::Right,
            },
            Rot(0.),
            RotVel(0.),
            Vel(vec2(0., 0.)),
            Pos(vec2(0.0, 0.0)),
            Grav,
            Drag,
            Boost(0.0),
            color_pallet()[0],
            Collides(Box::new(wireframe_to_polyline(PLAYER_WIREFRAME))),
            Wireframe(PLAYER_WIREFRAME),
            BoostToots {
                spawner: Arc::new(firetrail),
                every: 1.0 / 60.,
                next_toot: get_time(),
            },
        ));

        // player 2
        world.spawn((
            Controls {
                up: KeyCode::W,
                down: KeyCode::S,
                left: KeyCode::A,
                right: KeyCode::D,
            },
            Rot(0.),
            RotVel(0.),
            Vel((0., 0.).into()),
            Pos(vec2(PLAYER_SIZE, PLAYER_SIZE)),
            Grav,
            Drag,
            Boost(0.0),
            color_pallet()[1],
            Collides(Box::new(wireframe_to_polyline(PLAYER_WIREFRAME))),
            Wireframe(PLAYER_WIREFRAME),
            BoostToots {
                spawner: Arc::new(firetrail),
                every: 1.0 / 60.,
                next_toot: get_time(),
            },
        ));

        let walls = (
            Collides(Box::new(wireframe_to_polyline(BOUNDS_WIREFRAME))),
            Wireframe(BOUNDS_WIREFRAME),
            Pos(vec2(0., 0.)),
            Vel(vec2(0., 0.)),
            Rot(0.),
            color_pallet()[2],
        );
        world.spawn(walls);

        world.spawn((Settings::default(),));
        world.spawn((Background(color_pallet()[3]),));

        Self { world }
    }

    pub fn update(&mut self) {
        self.controls();
        self.collision();
        self.newtonian();
        BoostToots::system(&mut self.world);
        Mortal::system(&mut self.world);
        self.msc();
    }

    pub fn ui(&mut self) {
        let ui: &mut Ui = &mut root_ui();

        for (_, (settings,)) in self.world.query::<(&mut Settings,)>().iter() {
            Window::new(hash!(), vec2(10.0, 40.0), vec2(400.0, 500.0)).ui(ui, |ui| settings.ui(ui));
        }
    }

    pub fn draw(&self) {
        clear_background(self.get_one::<Background>().unwrap_or(Background(GRAY)).0);
        for (_, (camera,)) in self.world.query::<(&Camera2D,)>().iter() {
            set_camera(camera);
            for (_, (c, p, r, w)) in self
                .world
                .query::<(&Color, &Pos, &Rot, &Wireframe)>()
                .iter()
            {
                let q = r.quat();
                draw_wireframe(w.0, p.0, q, *c);
            }
        }

        set_default_camera();
        draw_text(&format!("fps: {}", get_fps()), 10.0, 30.0, 30.0, WHITE);
    }

    pub fn should_quit(&self) -> bool {
        self.world.query::<(&Quit,)>().into_iter().next().is_some()
    }

    /// panics if there is more than one entity with component T
    fn get_one<T: 'static + Clone + Sync + Send>(&self) -> Option<T> {
        let count = self.world.query::<(&T,)>().iter().count();
        assert!(count <= 1);
        self.world
            .query::<(&T,)>()
            .iter()
            .next()
            .map(|(_, (t,))| t.clone())
    }

    fn get_settings(&mut self) -> Settings {
        let count = self.world.query_mut::<(&Settings,)>().into_iter().count();
        assert!(count <= 1);
        self.world
            .query_mut::<(&Settings,)>()
            .into_iter()
            .next()
            .map(|(_, (s,))| s.clone())
            .unwrap_or_default()
    }
}

impl Heli {
    fn controls(&mut self) {
        let delta_t = get_frame_time();
        let settings = self.get_settings();

        // boost
        for (_id, (controls, Boost(b))) in self.world.query_mut::<(&Controls, &mut Boost)>() {
            *b = 0.0;
            if is_key_down(controls.up) {
                *b += settings.boost_power;
            }
            if is_key_down(controls.down) {
                *b -= settings.boost_power;
            }
        }

        // rotation accel
        for (_id, (controls, RotVel(rv))) in self.world.query_mut::<(&Controls, &mut RotVel)>() {
            if is_key_down(controls.left) {
                *rv += settings.rotational_acceleration * delta_t;
            }
            if is_key_down(controls.right) {
                *rv -= settings.rotational_acceleration * delta_t;
            }
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
                let rotvel_delta = -r * settings.auto_up_power * delta_t;
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
        let settings = self.get_settings();

        let mut collisions: Vec<(Entity, Entity, TOI, f32)> = Vec::new();
        for (ia, (Vel(va), Pos(pa), Collides(ca), Rot(ra))) in
            self.world.query::<(&Vel, &Pos, &Collides, &Rot)>().iter()
        {
            for (ib, (Vel(vb), Pos(pb), Collides(cb), Rot(rb))) in
                self.world.query::<(&Vel, &Pos, &Collides, &Rot)>().iter()
            {
                if ia == ib {
                    continue;
                }
                let impact = time_of_impact(
                    &Isometry::new([pa.x, pa.y].into(), *ra),
                    &Vector::new(va.x, va.y),
                    &**ca,
                    &Isometry::new([pb.x, pb.y].into(), *rb),
                    &Vector::new(vb.x, vb.y),
                    &**cb,
                    delta_t,
                )
                .unwrap();
                match impact {
                    None => {}
                    Some(toi) => {
                        debug_assert!(toi.status != TOIStatus::Failed);
                        collisions.push((ia, ib, toi, *ra));
                    }
                }
            }
        }
        collisions.sort_by(|a, b| {
            (a.0, a.1, a.2.toi)
                .partial_cmp(&(b.0, b.1, b.2.toi))
                .unwrap()
        });
        collisions.dedup_by(|a, b| (a.0, a.1) == (b.0, b.1));
        for (
            ia,
            _ib,
            TOI {
                toi,
                witness1,
                witness2: _,
                normal1,
                normal2: _,
                status: _,
            },
            ra,
        ) in collisions
        {
            let (vel, pos, rvel) = self
                .world
                .query_one_mut::<(&mut Vel, &mut Pos, Option<&mut RotVel>)>(ia)
                .unwrap();
            let v: &mut Vec2 = &mut vel.0;
            let p: &mut Vec2 = &mut pos.0;
            let n: Vec2 = (normal1.x, normal1.y).into();
            // get normal in world space
            let n = Rot(ra).quat().mul_vec3(n.extend(0.0)).truncate();

            let mut rvel_bak = RotVel(0.0);
            let rvel = rvel.unwrap_or(&mut rvel_bak);

            // perhaps if this ends up being janky, you can use witness1 to calculate normal

            if v.dot(n) <= 0.0 {
                // velocity is already pointing away from normal
                // no need to bounce
                continue;
            }

            // reflect velocity according to normal
            // https://www.youtube.com/watch?v=naaeH1qbjdQ
            let newvel = *v - v.dot(n) * n * 2.0;

            // position is moved into the collision such that the next time velocity is applied
            // position will be outside of the collision
            // this makes for a fully elastic collision
            *p += (*v - newvel) * toi;

            rvel.0 += rotvel_delta_on_impact(
                Vec2::new(witness1.x, witness1.y),
                Vec2::new(normal1.x, normal1.y) * v.length(),
            );

            *v = newvel;

            // but wait, there's more. we now want to lose some energy
            *p = *p + *v * delta_t; // move pos to where we know it will be at end of tick
            *v = *v * (1.0 - settings.collision_energy_loss);
            *p = *p - *v * delta_t; // move p back so that is properly placed at end of tick
        }

        // collision with air, also known as drag
        let drag_mult = delta_t * settings.drag_coefficient;
        debug_assert!(drag_mult < 1.0);
        for (_i, (v, Drag)) in self.world.query_mut::<(&mut Vel, &Drag)>() {
            v.0 -= v.0 * drag_mult;
        }

        // rotational drag
        let rdrag_mult = delta_t * settings.rotational_drag_coefficient;
        debug_assert!(rdrag_mult < 1.0);
        for (_i, (rv, Drag)) in self.world.query_mut::<(&mut RotVel, &Drag)>() {
            rv.0 -= rv.0 * rdrag_mult;
        }
    }

    fn newtonian(&mut self) {
        let delta_t = get_frame_time();
        let settings = self.get_settings();

        // apply velocity to position
        for (_id, (v, p)) in self.world.query_mut::<(&Vel, &mut Pos)>() {
            p.0 += v.0 * delta_t;
        }

        // apply gravity to velocity
        for (_id, (Grav, v)) in self.world.query_mut::<(&Grav, &mut Vel)>() {
            v.0.y += settings.gravity * delta_t;
        }

        // apply boost to velocity
        for (_id, (b, r, v)) in self.world.query_mut::<(&Boost, &Rot, &mut Vel)>() {
            v.0 += r.quat().mul_vec3(Vec3::Y * b.0 * delta_t).truncate();
        }

        // apply rotational velocity to rotation
        for (_id, (rv, r)) in self.world.query_mut::<(&RotVel, &mut Rot)>() {
            r.0 += rv.0 * delta_t;
        }
    }

    fn msc(&mut self) {
        // maintain aspect ratio
        for (_, (camera,)) in self.world.query_mut::<(&mut Camera2D,)>() {
            camera.zoom = vec2(1., screen_width() / screen_height());
        }
    }
}

#[derive(Debug)]
pub struct Controls {
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
}

#[derive(Debug)]
pub struct Rot(pub f32);

impl Rot {
    pub fn quat(&self) -> Quat {
        Quat::from_rotation_z(self.0)
    }
}

#[derive(Debug)]
pub struct RotVel(pub f32);

#[derive(Debug, Clone)]
pub struct Vel(pub Vec2);

#[derive(Debug, Clone)]
pub struct Pos(pub Vec2);

#[derive(Debug)]
pub struct Grav;

#[derive(Debug)]
pub struct Drag;

#[derive(Debug, Clone)]
pub struct Boost(pub f32);

#[derive(Debug)]
pub struct Quit;

pub struct Collides(pub Box<dyn Shape>);

#[derive(Debug)]
pub struct Wireframe(pub &'static [(f32, f32)]);

#[derive(Debug, Clone)]
pub struct Background(pub Color);

fn draw_wireframe(wireframe: &[(f32, f32)], position: Vec2, rotation: Quat, color: Color) {
    debug_assert!(!wireframe.is_empty());
    // the screen is 2 units tall (-1.0 to 1.0)
    // the world is WORLD_HEIGHT meters wide
    let meters_per_screen = 1.0 / WORLD_HEIGHT;
    let line_width_meters = 1.0;

    let to_screen = |point: (f32, f32)| {
        let mut p: Vec2 = point.into();
        p = rotation.mul_vec3(p.extend(0.0)).truncate();
        p += position;
        p *= meters_per_screen;
        p
    };

    let screen_coords = wireframe.into_iter().cloned().map(to_screen);
    for (a, b) in screen_coords.clone().zip(screen_coords.skip(1)) {
        draw_line(
            a.x,
            a.y,
            b.x,
            b.y,
            meters_per_screen * line_width_meters,
            color,
        );
    }
}

fn rotvel_delta_on_impact(local_space_impact_point: Vec2, impulse_vector: Vec2) -> f32 {
    const MOMENT_OF_INERTIA: f32 = 10.0;

    -local_space_impact_point
        .normalize()
        .perp_dot(impulse_vector.normalize())
        * impulse_vector.length()
        / MOMENT_OF_INERTIA
}

use crate::constants::TRASH_WIREFRAME;
use crate::mortal::Mortal;
use crate::system::Boost;
use alloc::sync::Arc;
use hecs::Entity;
use macroquad::prelude::*;

pub type Spawner = Arc<dyn Fn(&mut hecs::World, Entity) + Send + Sync + 'static>;

/// Spawns new entities whenever boosting.
pub struct BoostToots {
    pub spawner: Spawner,
    pub every: f64,
    pub next_toot: f64,
}

impl BoostToots {
    pub fn system(w: &mut hecs::World) {
        let time = get_time();

        let mut emitting: Vec<(Entity, Spawner)> = Default::default();
        for (ent, (boost, bt)) in w.query_mut::<(&Boost, &mut BoostToots)>() {
            while bt.next_toot <= time {
                bt.next_toot += bt.every;
                if boost.0.abs() > 0.00001 {
                    emitting.push((ent, bt.spawner.clone()));
                }
            }
        }

        for (ent, spawner) in emitting {
            spawner(w, ent);
        }
    }
}

pub fn firetrail(w: &mut hecs::World, source: Entity) {
    use crate::system::*;
    let pos = match get::<Pos>(w, source) {
        Some(pos) => pos,
        None => {
            if cfg!(debug_assertions) {
                panic!("entity with no position was assigned a firetrail");
            }
            return;
        }
    }
    .0;
    let rot: f32 = w.get::<Rot>(source).map(|c| c.0).unwrap_or(0.0);
    let vel: Vec2 = get::<Vel>(w, source).unwrap_or(Vel(vec2(0.0, 0.0))).0;
    let color: Color = get::<Color>(w, source).unwrap_or(BLACK);
    let boost: f32 = get::<Boost>(w, source).unwrap_or(Boost(1.0)).0;

    let q = Rot(rot).quat();
    let rotate = |v: Vec2| q.mul_vec3(v.extend(0.)).truncate();

    w.spawn((
        Rot(rot),
        RotVel(15. * boost.signum()),
        Vel(vel + rotate(vec2(0.0, -200.0)) * boost.signum()),
        Pos(pos + rotate(vec2(0.0, -10.0)) * boost.signum()),
        Grav,
        Drag,
        color,
        Wireframe(TRASH_WIREFRAME),
        Mortal {
            erase_at: get_time() + 2.0,
        },
    ));
}

fn get<T: Clone + Send + Sync + 'static>(w: &hecs::World, ent: Entity) -> Option<T> {
    w.get::<T>(ent).ok().map(|c| -> T { (&c as &T).clone() })
}

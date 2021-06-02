use hecs::Entity;
use macroquad::prelude::*;

pub struct Mortal {
    pub erase_at: f64,
}

impl Mortal {
    pub fn system(w: &mut hecs::World) {
        let time = get_time();

        let mut eol: Vec<Entity> = Vec::new();
        for (ent, (m,)) in w.query_mut::<(&Mortal,)>() {
            if m.erase_at <= time {
                eol.push(ent);
            }
        }

        for ent in eol {
            w.despawn(ent).unwrap();
        }
    }
}

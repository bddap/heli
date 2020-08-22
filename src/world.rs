use crate::mesh;
use crate::system;
use core::fmt::Debug;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::set_screen_coordinates;
use ggez::graphics::window;
use ggez::graphics::Color;
use ggez::graphics::Mesh;
use ggez::graphics::Rect;
use ggez::Context;
use ggez::GameResult;

#[derive(Default, Debug)]
pub struct World {
    pub pos: Vec<Option<(f32, f32)>>,
    pub vel: Vec<Option<(f32, f32)>>,
    pub rot: Vec<Option<f32>>,
    pub draw: Vec<Option<Mesh>>,
    // the entity has a bounding box
    pub bb: Vec<Option<()>>,
    // the entity is effected by gravity
    pub grav: Vec<Option<()>>,
}

impl World {
    pub const TICKS_PER_SECOND: usize = 128;
    pub const GRAVITY: f32 = -9. / (World::TICKS_PER_SECOND as f32);
    pub const METERS_TALL: f32 = 32.;

    pub fn new(ctx: &mut Context) -> World {
        let mut ret: World = Default::default();
        ret.spawn_player(ctx);
        ret.assert_valid();
        ret
    }

    fn spawn_player(&mut self, ctx: &mut Context) {
        self.create_entity(Entity {
            pos: Some((0.0, Self::METERS_TALL / 2.)),
            vel: Some((0.0, 0.0)),
            rot: Some(0.0),
            draw: Some(mesh::make_triangle(ctx, Color::new(0.3, 0.5, 0.4, 1.0)).unwrap()),
            bb: Some(()),
            grav: Some(()),
        });
    }

    fn create_entity(&mut self, entity: Entity) {
        let Entity {
            pos,
            vel,
            rot,
            draw,
            bb,
            grav,
        } = entity;
        self.pos.push(pos);
        self.vel.push(vel);
        self.rot.push(rot);
        self.draw.push(draw);
        self.bb.push(bb);
        self.grav.push(grav);
    }

    fn assert_valid(&self) {
        let World {
            pos,
            vel,
            rot,
            draw,
            bb,
            grav,
        } = self;
        assert_eq!(pos.len(), vel.len());
        assert_eq!(pos.len(), rot.len());
        assert_eq!(pos.len(), draw.len());
        assert_eq!(pos.len(), bb.len());
        assert_eq!(pos.len(), grav.len());
    }
}

struct Entity {
    pos: Option<(f32, f32)>,
    vel: Option<(f32, f32)>,
    rot: Option<f32>,
    draw: Option<Mesh>,
    bb: Option<()>,
    grav: Option<()>,
}

impl EventHandler for World {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        system::tick(self);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let aspect_ratio = window(ctx)
            .get_inner_size()
            .map(|ls| {
                debug_assert!(ls.height.abs() > 0.9);
                ls.width / ls.height
            })
            .unwrap_or(1.0);
        let meters_wide = Self::METERS_TALL * aspect_ratio as f32;
        set_screen_coordinates(
            ctx,
            Rect {
                x: -meters_wide / 2.,
                y: Self::METERS_TALL,
                w: meters_wide,
                h: -Self::METERS_TALL,
            },
        )?;
        graphics::clear(ctx, graphics::BLACK);
        system::draw(self, ctx)?;
        graphics::present(ctx)
    }
}

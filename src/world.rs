use crate::mesh;
use crate::system;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::Mesh;
use ggez::Context;
use ggez::GameResult;

pub struct World {
    pub pos: Vec<Option<(f32, f32)>>,
    pub vel: Vec<Option<(f32, f32)>>,
    pub rot: Vec<Option<f32>>,
    pub draw: Vec<Option<Mesh>>,
    pub ground: Vec<Option<()>>,
}

impl World {
    pub const GRAVITY: f32 = -1.0;
    pub fn new(ctx: &mut Context) -> World {
        let mut ret = World {
            pos: vec![],
            vel: vec![],
            rot: vec![],
            draw: vec![],
            ground: vec![],
        };

        ret.spawn_player(ctx);

        ret.assert_valid();
        ret
    }

    fn spawn_player(&mut self, ctx: &mut Context) {
        self.create_entity(Entity {
            pos: Some((0.0, 0.0)),
            vel: Some((0.0, 0.0)),
            rot: Some(0.0),
            draw: Some(mesh::make_triangle(ctx, Color::new(0.3, 0.5, 0.4, 1.0)).unwrap()),
            ground: Some(()),
        });
    }

    fn create_entity(&mut self, entity: Entity) {
        let Entity {
            pos,
            vel,
            rot,
            draw,
            ground,
        } = entity;
        self.pos.push(pos);
        self.vel.push(vel);
        self.rot.push(rot);
        self.draw.push(draw);
        self.ground.push(ground);
    }

    fn assert_valid(&self) {
        let World {
            pos,
            vel,
            rot,
            draw,
            ground,
        } = self;
        assert_eq!(pos.len(), vel.len());
        assert_eq!(pos.len(), rot.len());
        assert_eq!(pos.len(), draw.len());
        assert_eq!(pos.len(), ground.len());
    }
}

struct Entity {
    pos: Option<(f32, f32)>,
    vel: Option<(f32, f32)>,
    rot: Option<f32>,
    draw: Option<Mesh>,
    ground: Option<()>,
}

impl EventHandler for World {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        system::tick(self);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        system::draw(self, ctx)?;
        graphics::present(ctx)
    }
}

use ggez::{
    graphics::{self, Color, DrawMode},
    Context, GameResult,
};

pub fn make_triangle(ctx: &mut Context, color: Color) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();
    mb.circle(DrawMode::fill(), [0.0, 0.0], 1.0, 1.0, color);
    mb.build(ctx)
}

use ggez::{
    graphics::{self, Color},
    Context, GameResult,
};

pub fn make_triangle(ctx: &mut Context, color: Color) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();
    mb.triangles(&[[-0.5, 0.0], [0.0, 0.75], [0.5, 0.0]], color)?;
    mb.build(ctx)
}

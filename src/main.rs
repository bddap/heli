use bevy::prelude::*;

mod constants;
mod setup;
mod system;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugin(system::Heli)
        .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup::setup.system())
        // .add_system(paddle_movement_system.system())
        // .add_system(ball_collision_system.system())
        // .add_system(ball_movement_system.system())
        // .add_system(scoreboard_system.system())
        // .add_system(target_color.system())
        .run();
}

// fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
//     let aspect_ratio = window(ctx)
//         .get_inner_size()
//         .map(|ls| {
//             debug_assert!(ls.height.abs() > 0.9);
//             ls.width / ls.height
//         })
//         .unwrap_or(1.0);
//     let meters_wide = Self::METERS_TALL * aspect_ratio as f32;
//     set_screen_coordinates(
//         ctx,
//         Rect {
//             x: -meters_wide / 2.,
//             y: Self::METERS_TALL,
//             w: meters_wide,
//             h: -Self::METERS_TALL,
//         },
//     )?;
//     graphics::clear(ctx, graphics::BLACK);
//     system::draw(self, ctx)?;
//     graphics::present(ctx)
// }

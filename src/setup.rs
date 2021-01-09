use bevy::prelude::{
    Assets, Camera2dBundle, Color, ColorMaterial, Commands, ResMut, Sprite, SpriteBundle,
    Transform, Vec2, Vec3,
};

pub fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        // cameras
        .spawn(Camera2dBundle {
            transform: crate::constants::CAMERA_SETUP,
            ..Camera2dBundle::default()
        })
        // heli
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(1.1, 1.0)),
            ..Default::default()
        })
        .with(crate::system::Controls)
        .with(crate::system::Rot(0.0))
        .with(crate::system::RotVel(0.0))
        .with(crate::system::Vel(Vec2::zero()))
        .with(crate::system::Pos(Vec2::zero()))
        .with(crate::system::Grav)
        .with(crate::system::Drag)
        .with(crate::system::Boost(0.0));
}

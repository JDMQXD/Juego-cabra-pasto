use bevy::prelude::*;
use crate::components::goat::Goat;

pub fn spawn_goat(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            //Translation, rotation, scale
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
        Goat,
    ));
}

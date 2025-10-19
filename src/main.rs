use bevy::prelude::*;

mod components;
mod entities;
mod systems;
mod plugins;

use components::grid::GridConfig;
use components::goat::GoatState;
use entities::{spawn_grass, spawn_goat};
use systems::eating::goat_eat_grass_system;
use plugins::sound_plugin::SoundPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .insert_resource(GridConfig::default())
        .insert_resource(GoatState::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cabra y Pasto".to_string(),
                resolution: (800.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SoundPlugin) // nuestro plugin
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, (spawn_grass::spawn_grass_grid, spawn_goat::spawn_goat))
        .add_systems(Update, (goat_movement_system, goat_eat_grass_system))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Sistema de movimiento (igual)
fn goat_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<components::goat::Goat>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    for mut transform in &mut query {
        transform.translation += direction * 200.0 * time.delta_seconds();
    }
}

use bevy::prelude::*;
use crate::components::grid::{Grass, GridConfig};

pub fn spawn_grass_grid(mut commands: Commands, grid: Res<GridConfig>) {
    let start_x = -(grid.width as f32 / 2.0) * grid.cell_size;
    let start_y = -(grid.height as f32 / 2.0) * grid.cell_size;

    for i in 0..grid.width {
        for j in 0..grid.height {
            let x = start_x + i as f32 * grid.cell_size;
            let y = start_y + j as f32 * grid.cell_size;

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.0, 1.0, 0.0),
                        custom_size: Some(Vec2::splat(grid.cell_size * 0.9)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                    ..default()
                },
                Grass,
            ));
        }
    }
}

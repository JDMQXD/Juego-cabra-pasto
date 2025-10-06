use bevy::prelude::*;

#[derive(Resource)]
pub struct GridConfig {
    pub cell_size: f32,
    pub width: usize,
    pub height: usize,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            cell_size: 40.0,
            width: 10,
            height: 10,
        }
    }
}

#[derive(Component)]
pub struct Grass;

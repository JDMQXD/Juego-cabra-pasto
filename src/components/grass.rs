use bevy::prelude::*;

#[derive(Component)]
pub struct Grass {
    pub eaten: bool,
}

impl Default for Grass {
    fn default() -> Self {
        Self { eaten: false }
    }
}

#[derive(Component)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}



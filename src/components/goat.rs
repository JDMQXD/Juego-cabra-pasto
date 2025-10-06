use bevy::prelude::*;

#[derive(Component)]
pub struct Goat;

#[derive(Resource, Default)]
pub struct GoatState {
    pub hunger: f32,
}

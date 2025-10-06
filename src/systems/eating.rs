use bevy::prelude::*;
use crate::components::grid::Grass;
use crate::components::goat::{Goat, GoatState};

pub fn goat_eat_grass_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut goat_query: Query<&Transform, With<Goat>>,
    grass_query: Query<(Entity, &Transform), With<Grass>>,
    mut goat_state: ResMut<GoatState>,
) {
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    if let Ok(goat_transform) = goat_query.get_single_mut() {
        let goat_pos = goat_transform.translation.truncate();

        for (grass_entity, grass_transform) in &grass_query {
            let grass_pos = grass_transform.translation.truncate();

            if goat_pos.distance(grass_pos) < 30.0 {
                commands.entity(grass_entity).despawn();
                goat_state.hunger += 10.0;
                break;
            }
        }
    }
}

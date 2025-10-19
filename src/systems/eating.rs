use bevy::prelude::*;
use crate::components::grid::Grass;
use crate::components::goat::{Goat, GoatState};
use crate::plugins::sound_plugin::GoatAteGrassEvent;

pub fn goat_eat_grass_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    goat_query: Query<&Transform, With<Goat>>,
    grass_query: Query<(Entity, &Transform), With<Grass>>,
    mut goat_state: ResMut<GoatState>,
    mut event_writer: EventWriter<GoatAteGrassEvent>,
) {
    // Solo se ejecuta si el jugador presiona la tecla E
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    // Si hay una cabra en la escena
    if let Ok(goat_transform) = goat_query.get_single() {
        let goat_pos = goat_transform.translation.truncate();

        // Recorremos todo el pasto para ver si hay alguno cerca
        for (grass_entity, grass_transform) in &grass_query {
            let grass_pos = grass_transform.translation.truncate();

            // Si la cabra está lo suficientemente cerca del pasto (distancia < 30)
            if goat_pos.distance(grass_pos) < 30.0 {
                // Quitamos el pasto
                commands.entity(grass_entity).despawn();

                // Aumentamos la "energía" o "hambre" satisfecha
                goat_state.hunger = (goat_state.hunger + 10.0).clamp(0.0, 100.0);

                // Emitimos el evento para reproducir el sonido
                event_writer.send(GoatAteGrassEvent);

                break; // Salimos del bucle, solo come uno a la vez
            }
        }
    }
}

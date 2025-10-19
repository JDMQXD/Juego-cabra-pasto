use bevy::prelude::*;
use bevy::audio::{AudioSource, PlaybackSettings};

#[derive(Event)]
pub struct GoatAteGrassEvent;

#[derive(Resource)]
pub struct EatSound {
    pub handle: Handle<AudioSource>,
}

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        // No hace falta volver a agregar AudioPlugin porque ya está en DefaultPlugins
        app
            // Registrar el tipo de asset de audio (necesario para cargar archivos .ogg)
            .init_asset::<AudioSource>()
            // Registrar eventos y sistemas
            .add_event::<GoatAteGrassEvent>()
            .add_systems(Startup, load_eat_sound)
            .add_systems(Update, play_eat_sound);
    }
}

fn load_eat_sound(asset_server: Res<AssetServer>, mut commands: Commands) {
    // La ruta debe ser relativa a la carpeta "assets"
    let handle = asset_server.load("NOM.ogg");
    commands.insert_resource(EatSound { handle });
}

fn play_eat_sound(
    mut events: EventReader<GoatAteGrassEvent>,
    audio_assets: Res<EatSound>,
    mut commands: Commands,
) {
    for _ in events.read() {
        commands.spawn(AudioSourceBundle {
            source: audio_assets.handle.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

use bevy::prelude::*;

pub struct InternalAudioPlugin;
impl Plugin for InternalAudioPlugin {
    fn name(&self) -> &str {
        "Internal Audio Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup() {}

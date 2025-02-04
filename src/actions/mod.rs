use bevy::prelude::*;

pub struct ActionsPlugin;
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup() {}

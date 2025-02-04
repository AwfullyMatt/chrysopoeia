use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;
impl Plugin for PlayerPlugin {
    fn name(&self) -> &str {
        "Player Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup() {}

use bevy::prelude::*;

use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn name(&self) -> &str {
        "Player Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(OnEnter(GameState::Loading), spawn_player);
    }
}

fn startup() {}

fn spawn_player(mut commands: Commands) {
    commands.spawn(Player);
    info!("[SPAWNED] Player");
}

#[derive(Component)]
pub struct Player;

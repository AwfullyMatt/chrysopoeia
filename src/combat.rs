use crate::CombatState;
use bevy::prelude::*;

pub struct CombatPlugin;
impl Plugin for CombatPlugin {
    fn name(&self) -> &str {
        "Combat Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CombatState::In), startup)
            .add_systems(OnExit(CombatState::Out), cleanup);
    }
}

#[derive(Component)]
struct CleanupCombat;

fn startup() {
    info!("[STARTUP] Combat");
}

fn cleanup(mut commands: Commands, query_cleanup: Query<Entity, With<CleanupCombat>>) {
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
        info!("[CLEANUP] Combat");
    }
}

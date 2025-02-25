use bevy::prelude::*;

use crate::{loading::TextureAssets, settings::Settings, CombatState};

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

fn startup(mut commands: Commands, texture_assets: Res<TextureAssets>, settings: Res<Settings>) {
    let entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(20.0),
                top: Val::Percent(80.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,

                ..default()
            },
            CleanupCombat,
        ))
        .id();

    for _i in 0..4 {
        let child = commands
            .spawn((
                ImageNode::from_atlas_image(
                    texture_assets.button_atlas.clone(),
                    TextureAtlas::from(texture_assets.button_layout.clone()),
                ),
                Node {
                    width: Val::Px(40. * settings.resolution.scale.scale()),
                    height: Val::Px(40. * settings.resolution.scale.scale()),
                    ..default()
                },
            ))
            .id();

        commands.entity(entity).add_child(child);
    }

    info!("[STARTUP] Combat");
}

fn cleanup(mut commands: Commands, query_cleanup: Query<Entity, With<CleanupCombat>>) {
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
        info!("[CLEANUP] Combat");
    }
}

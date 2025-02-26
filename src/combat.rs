use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    actions::CombatButtonAction, loading::TextureAssets, settings::Settings, CombatState, GameState,
};

pub struct CombatPlugin;
impl Plugin for CombatPlugin {
    fn name(&self) -> &str {
        "Combat Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CombatState::In), startup)
            .add_systems(
                Update,
                combat_button_press.run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(CombatState::Out), cleanup);
    }
}

#[derive(Component)]
struct CleanupCombat;

#[derive(Component, Deref, DerefMut)]
struct CombatButton(ButtonRow);

#[derive(Component, Deref, DerefMut)]
struct ButtonRow(usize);

fn startup(mut commands: Commands, texture_assets: Res<TextureAssets>, settings: Res<Settings>) {
    let entity = commands
        .spawn((
            Name::new("Combat Button Node"),
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

    for i in 0..4 {
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
                CombatButton(ButtonRow(i)),
            ))
            .id();

        commands.entity(entity).add_child(child);
        info!("[SPAWNED] Combat Button: {i}");
    }

    info!("[STARTUP] Combat");
}

fn cleanup(mut commands: Commands, query_cleanup: Query<Entity, With<CleanupCombat>>) {
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
        info!("[CLEANUP] Combat");
    }
}

fn combat_button_press(
    mut query_button_node: Query<(&mut ImageNode, &CombatButton)>,
    query_button_action: Query<&ActionState<CombatButtonAction>>,
) {
    if let Ok(action_state) = query_button_action.get_single() {
        for button in CombatButtonAction::array() {
            if action_state.just_pressed(&button) {
                info!("[PRESSED] Button: {button:?}");
                for (mut button_node, combat_button) in &mut query_button_node {
                    if ***combat_button == button.index() {
                        if let Some(atlas) = &mut button_node.texture_atlas {
                            atlas.index = (atlas.index + 2) % 3;
                        }
                    }
                }
            }
            if action_state.just_released(&button) {
                info!("[RELEASED] Button: {button:?}");
                for (mut button_node, combat_button) in &mut query_button_node {
                    if ***combat_button == button.index() {
                        if let Some(atlas) = &mut button_node.texture_atlas {
                            atlas.index = (atlas.index - 2) % 3;
                        }
                    }
                }
            }
        }
    }
}

use crate::loading::UiAssets;
use crate::settings::Settings;
use crate::ui::{
    UiBackgroundColor, UiBorderColor, UiButton, UiButtonNode, UiButtonRow, UiButtonState,
    UiParentNode, UiParentNodePosition, UiTextColor,
};
use crate::{CombatState, GameState};
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), startup)
            .add_systems(Update, click_ui_buttons.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup);
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
#[allow(dead_code)] // TODO:
enum MainMenuButton {
    Play,
    Settings,
    Exit,
    Github,
    Bevy,
}

#[derive(Component)]
struct CleanupMainMenu;

fn startup(mut commands: Commands, ui: Res<UiAssets>, settings: Res<Settings>) {
    info!("[STARUP] Main Menu");
    commands.spawn((Camera2d, Msaa::Off));
    let style = (
        BackgroundColor(UiBackgroundColor::default().normal.srgb()),
        BorderColor(UiBorderColor::default().normal.srgb()),
        BorderRadius::ZERO,
    );

    // Spawn Center/Left/Right Nodes
    commands.spawn((
        Name::new("UI Parent Node: Right"),
        UiParentNode::new(UiParentNodePosition::Right),
    ));
    // .with_child(ImageNode::from_atlas_image(
    //     ui.pendulum_atlas.clone(),
    //     TextureAtlas {
    //         layout: ui.pendulum_layout.clone(),
    //         index: 0,
    //     },
    // ));
    commands.spawn((
        Name::new("UI Parent Node: Left"),
        UiParentNode::new(UiParentNodePosition::Left),
    ));
    let center_entity = commands
        .spawn((
            Name::new("UI Parent Node: Center"),
            UiParentNode::new(UiParentNodePosition::Center),
        ))
        .id();

    for i in 0..4 {
        let state = match i {
            0 => UiButtonState::Confirm,
            1 => UiButtonState::Deny,
            2 => UiButtonState::Settings,
            3 => UiButtonState::Misc,
            _ => unreachable!("[ERROR] UI Button Index Outside of Range."),
        };
        let child = commands
            .spawn((
                ImageNode::from_atlas_image(
                    ui.button_icon_atlas.clone(),
                    TextureAtlas {
                        layout: ui.button_icon_layout.clone(),
                        index: state.index(),
                    },
                ),
                Node {
                    width: Val::Px(40. * settings.resolution.scale.scale()),
                    height: Val::Px(40. * settings.resolution.scale.scale()),
                    ..default()
                },
                UiButton(UiButtonRow(i)),
                state,
            ))
            .id();

        commands.entity(center_entity).add_child(child);
        info!("[SPAWNED] UI Button: {i}");
    }

    commands
        .spawn((
            Name::new("Bottom Parent Node"),
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                bottom: Val::Px(5.),
                width: Val::Percent(100.),
                position_type: PositionType::Absolute,
                ..default()
            },
            MainMenu,
            CleanupMainMenu,
        ))
        .with_children(|children| {
            children
                .spawn((
                    Name::new("Bevy Logo Child Node"),
                    Button,
                    MainMenuButton::Bevy,
                    UiButtonNode::small(),
                    style.clone(),
                    OpenLink("https://bevyengine.org"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Bevy Logo Grandchild Node"),
                        Text::new("Made with Bevy"),
                        TextFont {
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(UiTextColor::default().normal.srgb()),
                    ));
                    parent.spawn((
                        ImageNode {
                            image: ui.bevy.clone(),
                            ..default()
                        },
                        Node {
                            width: Val::Px(32.),
                            ..default()
                        },
                    ));
                });
            children
                .spawn((
                    Name::new("Github Parent Node"),
                    Button,
                    MainMenuButton::Github,
                    UiButtonNode::small(),
                    style.clone(),
                    OpenLink("https://github.com/AwfullyMatt/chrysopoeia"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Github Text Node"),
                        Text::new("Github"),
                        TextFont {
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(UiTextColor::default().normal.srgb()),
                    ));
                    parent.spawn((
                        Name::new("Github Image Node"),
                        ImageNode::new(ui.github.clone()),
                        Node {
                            width: Val::Px(32.),
                            ..default()
                        },
                    ));
                });
        });
}

#[derive(Component)]
struct OpenLink(&'static str);

fn click_ui_buttons(
    mut game_state: ResMut<NextState<GameState>>,
    mut combat_state: ResMut<NextState<CombatState>>,
    mut interaction_query: Query<
        (&Interaction, &MainMenuButton, Option<&OpenLink>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mmb, open_link) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match mmb {
                MainMenuButton::Play => {
                    game_state.set(GameState::Playing);
                    combat_state.set(CombatState::In);
                }
                MainMenuButton::Settings => {}
                MainMenuButton::Exit => {}
                MainMenuButton::Github => {
                    if let Some(link) = open_link {
                        if let Err(error) = webbrowser::open(link.0) {
                            warn!("Failed to open link {error:?}");
                        }
                    }
                }
                MainMenuButton::Bevy => {
                    if let Some(link) = open_link {
                        if let Err(error) = webbrowser::open(link.0) {
                            warn!("Failed to open link {error:?}");
                        }
                    }
                }
            },
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn cleanup(mut commands: Commands, menu: Query<Entity, With<CleanupMainMenu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
        info!("[CLEANUP] Main Menu");
    }
}

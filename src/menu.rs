use crate::loading::TextureAssets;
use crate::ui::{UiBackgroundColor, UiBorderColor, UiButtonNode, UiParentNode, UiTextColor};
use crate::{CombatState, GameState};
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), startup)
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
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

fn startup(mut commands: Commands, textures: Res<TextureAssets>) {
    info!("[STARUP] Main Menu");
    commands.spawn((Camera2d, Msaa::Off));
    let style = (
        BackgroundColor(UiBackgroundColor::default().normal.srgb()),
        BorderColor(UiBorderColor::default().normal.srgb()),
        BorderRadius::ZERO,
    );
    commands
        .spawn((
            Name::new("Main Menu Parent Node"),
            UiParentNode::full(),
            MainMenu,
            CleanupMainMenu,
        ))
        .with_children(|children| {
            children
                .spawn((
                    Name::new("Play Button Node"),
                    Button,
                    MainMenuButton::Play,
                    UiButtonNode::normal(),
                    style.clone(),
                ))
                .with_child((
                    Text::new("Play"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(UiTextColor::default().normal.srgb()),
                ));
        });
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
                            image: textures.bevy.clone(),
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
                        ImageNode::new(textures.github.clone()),
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

fn click_play_button(
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
                    //combat_state.set(CombatState::In);
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

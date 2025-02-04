#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod loading;
mod menu;
mod player;
mod ui;

use std::io::Cursor;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
use bevy::asset::AssetMetaCheck;
/* #[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}; */
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ui::{Palette, UiPlugin};
use winit::window::Icon;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn name(&self) -> &str {
        "Game Plugin"
    }

    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::linear_rgb(0.4, 0.4, 0.4)))
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Chrysopoeia".to_string(),
                            // Bind to canvas included in `index.html`
                            canvas: Some("#chrysopoeia".to_owned()),
                            fit_canvas_to_parent: true,
                            // Tells wasm not to override default event handling, like F5 and Ctrl+R
                            prevent_default_event_handling: false,
                            window_theme: Some(bevy::window::WindowTheme::Dark),
                            resizable: false,
                            ..default()
                        }),
                        ..default()
                    })
                    .set(AssetPlugin {
                        meta_check: AssetMetaCheck::Never,
                        ..default()
                    })
                    .set(ImagePlugin::default_nearest()),
            )
            .add_plugins(WorldInspectorPlugin::new())
            .add_plugins((
                LoadingPlugin,
                MenuPlugin,
                ActionsPlugin,
                InternalAudioPlugin,
                PlayerPlugin,
                UiPlugin,
            ))
            .add_systems(Startup, startup)
            .init_state::<GameState>()
            .insert_resource(ClearColor(Palette::Dark.srgb()))
            .add_sub_state::<PauseState>();

        /* #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        } */
    }
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    Menu,
}

#[derive(SubStates, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[source(GameState = GameState::Playing)]
pub enum PauseState {
    #[default]
    Unpaused,
    Paused,
}

fn startup(windows: NonSend<WinitWindows>, primary_window: Query<Entity, With<PrimaryWindow>>) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}

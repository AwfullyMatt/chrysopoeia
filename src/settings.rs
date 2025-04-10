use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::GameState;

pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn name(&self) -> &str {
        "Settings Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(OnEnter(GameState::Settings), on_enter)
            .insert_resource(Settings::default());
    }
}

#[derive(Resource, Clone, Default, Serialize, Deserialize)]
pub struct Settings {
    pub resolution: Resolution,
    pub monitor: Option<usize>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Resolution {
    pub vec: Vec2,
    pub scale: ScaleFactor,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub enum ScaleFactor {
    Large,
    #[default]
    Small,
}
impl ScaleFactor {
    pub fn scale(&self) -> f32 {
        match self {
            ScaleFactor::Large => 4.,
            ScaleFactor::Small => 3.,
        }
    }
}

fn startup(settings: Res<Settings>, mut query_window: Query<&mut Window>) {
    if let Ok(mut window) = query_window.get_single_mut() {
        window.resolution.set(
            640. * settings.resolution.scale.scale(),
            360. * settings.resolution.scale.scale(),
        );
        if let Some(u) = settings.monitor {
            window.position.center(MonitorSelection::Index(u));
        } else {
            window.position.center(MonitorSelection::Primary);
        }
    }
}

fn on_enter() {}

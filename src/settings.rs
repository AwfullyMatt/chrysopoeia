use bevy::prelude::*;

use crate::GameState;

pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn name(&self) -> &str {
        "Settings Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Settings), startup);
    }
}

pub struct Settings {
    resolution: Resolution,
}

pub struct Resolution {
    vec: Vec2,
    scale: ScaleFactor,
}

pub enum ScaleFactor {
    Large,
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

fn startup() {}

// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use chrysopoeia::GamePlugin;

fn main() -> AppExit {
    App::new().add_plugins(GamePlugin).run()
}

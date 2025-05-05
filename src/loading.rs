use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn name(&self) -> &str {
        "Loading Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<UiAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "textures/ui/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/ui/github.png")]
    pub github: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 40, tile_size_y = 40, columns = 2, rows = 1,))]
    pub button_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "textures/ui/ui_button_atlas.png")]
    pub button_atlas: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 40, tile_size_y = 40, columns = 8, rows = 1,))]
    pub button_icon_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "textures/ui/button_icon_atlas.png")]
    pub button_icon_atlas: Handle<Image>,
}

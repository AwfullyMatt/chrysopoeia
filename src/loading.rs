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
                .load_collection::<TextureAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 40, tile_size_y = 40, columns = 3, rows = 1,))]
    pub button_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "textures/button_atlas.png")]
    pub button_atlas: Handle<Image>,
}

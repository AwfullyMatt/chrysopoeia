use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct ActionsPlugin;
impl Plugin for ActionsPlugin {
    fn name(&self) -> &str {
        "Actions Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_plugins(InputManagerPlugin::<UiButtonAction>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum UiButtonAction {
    One,
    Two,
    Three,
    Four,
}
impl UiButtonAction {
    fn init() -> InputMap<UiButtonAction> {
        InputMap::new([
            (UiButtonAction::One, KeyCode::KeyJ),
            (UiButtonAction::Two, KeyCode::KeyK),
            (UiButtonAction::Three, KeyCode::KeyL),
            (UiButtonAction::Four, KeyCode::Semicolon),
        ])
    }

    pub fn array() -> [UiButtonAction; 4] {
        [
            UiButtonAction::One,
            UiButtonAction::Two,
            UiButtonAction::Three,
            UiButtonAction::Four,
        ]
    }

    pub fn index(&self) -> usize {
        match self {
            Self::One => 0,
            Self::Two => 1,
            Self::Three => 2,
            Self::Four => 3,
        }
    }
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Name::new("Input Manager: Combat Buttons"),
        InputManagerBundle::with_map(UiButtonAction::init()),
    ));
}

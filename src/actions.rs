use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct ActionsPlugin;
impl Plugin for ActionsPlugin {
    fn name(&self) -> &str {
        "Actions Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_plugins(InputManagerPlugin::<CombatButtonAction>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CombatButtonAction {
    One,
    Two,
    Three,
    Four,
}
impl CombatButtonAction {
    fn init() -> InputMap<CombatButtonAction> {
        InputMap::new([
            (CombatButtonAction::One, KeyCode::KeyJ),
            (CombatButtonAction::Two, KeyCode::KeyK),
            (CombatButtonAction::Three, KeyCode::KeyL),
            (CombatButtonAction::Four, KeyCode::Semicolon),
        ])
    }

    pub fn array() -> [CombatButtonAction; 4] {
        [
            CombatButtonAction::One,
            CombatButtonAction::Two,
            CombatButtonAction::Three,
            CombatButtonAction::Four,
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
        InputManagerBundle::with_map(CombatButtonAction::init()),
    ));
}

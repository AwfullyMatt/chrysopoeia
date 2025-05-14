use bevy::prelude::*;

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn name(&self) -> &str {
        "Character Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

// DATA

#[derive(Component)]
pub struct CharacterStat {
    pub kind: StatKind,
    pub level: u8,
}

pub enum StatKind {
    Constitution,
    Agility,
    Occult,
    Nature,
    Social,
}

// SYSTEMS
fn startup() {}

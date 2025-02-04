use std::time::Duration;

use bevy::prelude::*;

pub struct InternalAudioPlugin;
impl Plugin for InternalAudioPlugin {
    fn name(&self) -> &str {
        "Internal Audio Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(FixedUpdate, tick_metronome)
            .init_resource::<CurrentMetre>()
            .init_resource::<CurrentTempo>();
    }
}

fn startup(mut commands: Commands) {
    commands.spawn(MetronomeBundle::new());
}

#[derive(Clone, Copy, Default, Resource)]
pub struct CurrentTempo(Option<Tempo>);
impl CurrentTempo {
    fn set(&mut self, tempo: Option<Tempo>) {
        self.0 = tempo;
    }

    fn get(&self) -> Option<Tempo> {
        self.0
    }
}

#[derive(Clone, Copy, Default, Resource)]
pub struct CurrentMetre(Option<Metre>);
impl CurrentMetre {
    fn set(&mut self, metre: Option<Metre>) {
        self.0 = metre;
    }

    fn get(&self) -> Option<Metre> {
        self.0
    }
}

#[derive(Bundle, Clone, Default)]
struct MetronomeBundle {
    name: Name,
    metronome: Metronome,
}
impl MetronomeBundle {
    fn new() -> Self {
        Self {
            name: Name::new("Metronome"),
            metronome: Metronome::new(),
        }
    }
}

#[derive(Component, Clone, Default)]
struct Metronome(Timer);
impl Metronome {
    fn new() -> Self {
        Metronome(Timer::new(
            Duration::from_secs_f64(0.0),
            TimerMode::Repeating,
        ))
    }
}

#[derive(Component, Clone, Copy, Deref, DerefMut)]
struct Tempo(u8);

#[derive(Component, Clone, Copy)]
struct Metre {
    top: u8,
    bottom: u8,
}

#[derive(Component)]
enum Note {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
}
impl Note {
    fn length(&self, curr_metre: Res<CurrentMetre>, curr_tempo: Res<CurrentTempo>) -> Option<f32> {
        use Note::*;

        if let Some(metre) = curr_metre.get() {
            if let Some(tempo) = curr_tempo.get() {
                let bps: f32 = *tempo as f32 / 60.0;
                match self {
                    Whole => {
                        let length = bps * metre.top as f32;
                        Some(length)
                    }
                    Half => {
                        let length = (bps * metre.top as f32) / 2.0;
                        Some(length)
                    }
                    Quarter => {
                        let length = (bps * metre.top as f32) / 4.0;
                        Some(length)
                    }
                    Eighth => {
                        let length = (bps * metre.top as f32) / 8.0;
                        Some(length)
                    }
                    Sixteenth => {
                        let length = (bps * metre.top as f32) / 16.0;
                        Some(length)
                    }
                    ThirtySecond => {
                        let length = (bps * metre.top as f32) / 32.0;
                        Some(length)
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn tick_metronome(time: Res<Time>, mut query_metronome: Query<&mut Metronome>) {
    if let Ok(mut metronome) = query_metronome.get_single_mut() {
        metronome.0.tick(time.delta());
    }
}

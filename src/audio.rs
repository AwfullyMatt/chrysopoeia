use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl, AudioSource};
use std::time::Duration;

pub struct InternalAudioPlugin;
impl Plugin for InternalAudioPlugin {
    fn name(&self) -> &str {
        "Internal Audio Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(FixedUpdate, tick_metronome)
            .add_systems(
                Update,
                (
                    evr_play_metronome_audio,
                    evr_pause_metronome_audio,
                    evr_stop_metronome_audio,
                    update_note_timers,
                ),
            )
            .init_resource::<CurrentSong>()
            .init_resource::<MetronomeAudioChannel>()
            .init_state::<MetronomeState>()
            .add_audio_channel::<MetronomeAudioChannel>()
            .add_event::<PlayMetronomeAudio>()
            .add_event::<PauseMetronomeAudio>()
            .add_event::<StopMetronomeAudio>();
    }
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum MetronomeState {
    #[default]
    Stopped,
    Paused,
    Playing,
}

fn startup(mut commands: Commands, current_song: Res<CurrentSong>) {
    if let Some(song) = &current_song.0 {
        commands.spawn(MetronomeBundle::new(&song.info));
    } else {
        commands.spawn(MetronomeBundle::default());
    }
    info!("[SPAWNED] Metronome");
}

#[derive(Clone, Copy, Default, Resource)]
struct MetronomeAudioChannel;

#[derive(Event, Deref)]
struct PlayMetronomeAudio(Song);

#[derive(Event)]
struct PauseMetronomeAudio;

#[derive(Event)]
struct StopMetronomeAudio;

#[derive(Component, Clone)]
struct Song {
    handle: Handle<AudioSource>,
    info: AudioInfo,
}

#[allow(dead_code)] //TODO
#[derive(Clone, Copy, Default)]
struct AudioInfo {
    tempo: Tempo,
    metre: Metre,
    intro: Option<AudioLength>,
    body: AudioLength,
    outro: Option<AudioLength>,
}
impl AudioInfo {}

#[derive(Clone, Copy, Default, Deref, DerefMut)]
struct AudioLength(f32);

#[derive(Clone, Default, Resource)]
struct CurrentSong(Option<Song>);
impl CurrentSong {}

#[derive(Bundle, Clone, Default)]
struct MetronomeBundle {
    name: Name,
    metronome: Metronome,
}
impl MetronomeBundle {
    fn new(audio_info: &AudioInfo) -> Self {
        Self {
            name: Name::new("Metronome"),
            metronome: Metronome::new(audio_info),
        }
    }
}

#[derive(Component, Clone, Default)]
struct Metronome {
    whole_note: NoteTimer,
    half_note: NoteTimer,
    quarter_note: NoteTimer,
    eighth_note: NoteTimer,
    sixteenth_note: NoteTimer,
    thirtysecond_note: NoteTimer,
}
impl Metronome {
    fn new(audio_info: &AudioInfo) -> Self {
        Metronome {
            whole_note: NoteTimer::new(NoteKind::Whole, audio_info),
            half_note: NoteTimer::new(NoteKind::Half, audio_info),
            quarter_note: NoteTimer::new(NoteKind::Quarter, audio_info),
            eighth_note: NoteTimer::new(NoteKind::Eighth, audio_info),
            sixteenth_note: NoteTimer::new(NoteKind::Sixteenth, audio_info),
            thirtysecond_note: NoteTimer::new(NoteKind::ThirtySecond, audio_info),
        }
    }

    fn update(&mut self, audio_info: &AudioInfo) {
        self.whole_note.update(audio_info);
        self.half_note.update(audio_info);
        self.quarter_note.update(audio_info);
        self.eighth_note.update(audio_info);
        self.sixteenth_note.update(audio_info);
        self.thirtysecond_note.update(audio_info);
    }
}

#[derive(Component, Clone, Default)]
struct NoteTimer {
    timer: Timer,
    kind: NoteKind,
}
impl NoteTimer {
    fn new(kind: NoteKind, audio_info: &AudioInfo) -> Self {
        Self {
            timer: Timer::new(
                Duration::from_secs_f32(kind.length(audio_info).unwrap_or_default()),
                TimerMode::Repeating,
            ),
            kind,
        }
    }

    fn update(&mut self, audio_info: &AudioInfo) {
        self.timer.set_duration(Duration::from_secs_f32(
            self.kind.length(audio_info).unwrap_or_default(),
        ));
    }
}

#[derive(Component, Clone, Copy, Default, Deref, DerefMut)]
struct Tempo(f32); // BPM
impl Tempo {}

#[derive(Component, Clone, Copy, Default)]
struct Metre {
    top: u8,
    bottom: u8,
}
impl Metre {}

#[derive(Component, Clone, Copy, Default)]
enum NoteKind {
    #[default]
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
}
impl NoteKind {
    fn length(&self, audio_info: &AudioInfo) -> Option<f32> {
        use NoteKind::*;
        let bps: f32 = audio_info.tempo.0 / 60.0;
        let spb: f32 = 1.0 / bps;
        let top: f32 = audio_info.metre.top as f32;
        match self {
            Whole => {
                let length = spb * top;
                Some(length)
            }
            Half => {
                let length = (spb * top) / 2.0;
                Some(length)
            }
            Quarter => {
                let length = (spb * top) / 4.0;
                Some(length)
            }
            Eighth => {
                let length = (spb * top) / 8.0;
                Some(length)
            }
            Sixteenth => {
                let length = (spb * top) / 16.0;
                Some(length)
            }
            ThirtySecond => {
                let length = (spb * top) / 32.0;
                Some(length)
            }
        }
    }
}

fn tick_metronome(time: Res<Time>, mut query_metronome: Query<&mut Metronome>) {
    if let Ok(mut metronome) = query_metronome.get_single_mut() {
        metronome.whole_note.timer.tick(time.delta());
        metronome.half_note.timer.tick(time.delta());
        metronome.quarter_note.timer.tick(time.delta());
        metronome.eighth_note.timer.tick(time.delta());
        metronome.sixteenth_note.timer.tick(time.delta());
        metronome.thirtysecond_note.timer.tick(time.delta());
    }
}

fn evr_play_metronome_audio(
    mut evr_play_metronome_audio: EventReader<PlayMetronomeAudio>,
    metronome_channel: Res<AudioChannel<MetronomeAudioChannel>>,
    metronome_state: Res<State<MetronomeState>>,
    mut next_metronome_state: ResMut<NextState<MetronomeState>>,
) {
    for ev in evr_play_metronome_audio.read() {
        metronome_channel.play(ev.0.handle.clone());
        if metronome_state.get() != &MetronomeState::Playing {
            next_metronome_state.set(MetronomeState::Playing);
        }
    }
}

fn evr_pause_metronome_audio(
    mut evr_pause_metronome_audio: EventReader<PauseMetronomeAudio>,
    metronome_channel: Res<AudioChannel<MetronomeAudioChannel>>,
    metronome_state: Res<State<MetronomeState>>,
    mut next_metronome_state: ResMut<NextState<MetronomeState>>,
) {
    for _ev in evr_pause_metronome_audio.read() {
        metronome_channel.pause();
        if metronome_state.get() != &MetronomeState::Paused {
            next_metronome_state.set(MetronomeState::Paused);
        }
    }
}

fn evr_stop_metronome_audio(
    mut evr_pause_metronome_audio: EventReader<StopMetronomeAudio>,
    metronome_channel: Res<AudioChannel<MetronomeAudioChannel>>,
    metronome_state: Res<State<MetronomeState>>,
    mut next_metronome_state: ResMut<NextState<MetronomeState>>,
) {
    for _ev in evr_pause_metronome_audio.read() {
        metronome_channel.pause();
        if metronome_state.get() != &MetronomeState::Stopped {
            next_metronome_state.set(MetronomeState::Stopped);
        }
    }
}
fn update_note_timers(current_song: Res<CurrentSong>, mut query_metronome: Query<&mut Metronome>) {
    if current_song.is_changed() {
        if let Some(song) = &current_song.0 {
            if let Ok(mut metronome) = query_metronome.get_single_mut() {
                metronome.update(&song.info);
            }
        }
    }
}

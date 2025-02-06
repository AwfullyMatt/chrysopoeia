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
                (evr_play_metronome_audio, evr_pause_metronome_audio),
            )
            .init_resource::<CurrentAudioInfo>()
            .init_resource::<MetronomeAudioChannel>()
            .add_audio_channel::<MetronomeAudioChannel>()
            .add_event::<PlayMetronomeAudio>()
            .add_event::<PauseMetronomeAudio>();
    }
}

fn startup(mut commands: Commands, current_audio_info: Res<CurrentAudioInfo>) {
    if let Some(audio_info) = current_audio_info.info() {
        commands.spawn(MetronomeBundle::new(&audio_info));
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
impl AudioInfo {
    pub fn tempo(&self) -> Tempo {
        self.tempo
    }
    pub fn metre(&self) -> Metre {
        self.metre
    }
    pub fn intro(&self) -> Option<AudioLength> {
        self.intro
    }
    pub fn body(&self) -> AudioLength {
        self.body
    }
    pub fn outro(&self) -> Option<AudioLength> {
        self.outro
    }
}

#[derive(Clone, Copy, Default, Deref, DerefMut)]
struct AudioLength(f32);

#[derive(Clone, Copy, Default, Resource)]
struct CurrentAudioInfo(Option<AudioInfo>);
impl CurrentAudioInfo {
    pub fn info(&self) -> Option<AudioInfo> {
        self.0
    }
}

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
    fn whole(&self) -> &NoteTimer {
        &self.whole_note
    }
    fn half(&self) -> &NoteTimer {
        &self.half_note
    }
    fn quarter(&self) -> &NoteTimer {
        &self.quarter_note
    }
    fn eighth(&self) -> &NoteTimer {
        &self.eighth_note
    }
    fn sixteenth(&self) -> &NoteTimer {
        &self.sixteenth_note
    }
    fn thirtysecond(&self) -> &NoteTimer {
        &self.thirtysecond_note
    }

    fn whole_mut(&mut self) -> &mut NoteTimer {
        &mut self.whole_note
    }
    fn half_mut(&mut self) -> &mut NoteTimer {
        &mut self.half_note
    }
    fn quarter_mut(&mut self) -> &mut NoteTimer {
        &mut self.quarter_note
    }
    fn eighth_mut(&mut self) -> &mut NoteTimer {
        &mut self.eighth_note
    }
    fn sixteenth_mut(&mut self) -> &mut NoteTimer {
        &mut self.sixteenth_note
    }
    fn thirtysecond_mut(&mut self) -> &mut NoteTimer {
        &mut self.thirtysecond_note
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
    fn timer_mut(&mut self) -> &mut Timer {
        &mut self.timer
    }
}

#[derive(Component, Clone, Copy, Default, Deref, DerefMut)]
struct Tempo(f32);
impl Tempo {
    fn get(&self) -> f32 {
        self.0
    }
}

#[derive(Component, Clone, Copy, Default)]
struct Metre {
    top: u8,
    bottom: u8,
}
impl Metre {
    fn top(&self) -> u8 {
        self.top
    }
    fn bottom(&self) -> u8 {
        self.bottom
    }
}

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
        let bps: f32 = audio_info.tempo().get() / 60.0;
        match self {
            Whole => {
                let length = bps * audio_info.metre().top() as f32;
                Some(length)
            }
            Half => {
                let length = (bps * audio_info.metre().top() as f32) / 2.0;
                Some(length)
            }
            Quarter => {
                let length = (bps * audio_info.metre().top() as f32) / 4.0;
                Some(length)
            }
            Eighth => {
                let length = (bps * audio_info.metre().top() as f32) / 8.0;
                Some(length)
            }
            Sixteenth => {
                let length = (bps * audio_info.metre().top() as f32) / 16.0;
                Some(length)
            }
            ThirtySecond => {
                let length = (bps * audio_info.metre().top() as f32) / 32.0;
                Some(length)
            }
        }
    }
}

fn tick_metronome(time: Res<Time>, mut query_metronome: Query<&mut Metronome>) {
    if let Ok(mut metronome) = query_metronome.get_single_mut() {
        metronome.whole_mut().timer_mut().tick(time.delta());
        metronome.half_mut().timer_mut().tick(time.delta());
        metronome.quarter_mut().timer_mut().tick(time.delta());
        metronome.eighth_mut().timer_mut().tick(time.delta());
        metronome.sixteenth_mut().timer_mut().tick(time.delta());
        metronome.thirtysecond_mut().timer_mut().tick(time.delta());
    }
}

fn evr_play_metronome_audio(
    mut evr_play_metronome_audio: EventReader<PlayMetronomeAudio>,
    metronome_channel: Res<AudioChannel<MetronomeAudioChannel>>,
) {
    for ev in evr_play_metronome_audio.read() {
        metronome_channel.play(ev.0.handle.clone());
    }
}

fn evr_pause_metronome_audio(
    mut evr_pause_metronome_audio: EventReader<PauseMetronomeAudio>,
    metronome_channel: Res<AudioChannel<MetronomeAudioChannel>>,
) {
    for _ev in evr_pause_metronome_audio.read() {
        metronome_channel.pause();
    }
}

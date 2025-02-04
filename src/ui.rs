use bevy::prelude::*;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn name(&self) -> &str {
        "UI Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, ui_button_interaction);
    }
}

fn startup() {}

pub struct UiParentNode;
impl UiParentNode {
    pub fn normal() -> Node {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }
    }
}

pub struct UiButtonNode;
impl UiButtonNode {
    pub fn normal() -> Node {
        Node {
            width: Val::Percent(10.0),
            height: Val::Percent(10.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        }
    }

    pub fn small() -> Node {
        Node {
            width: Val::Px(170.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(5.)),
            ..Default::default()
        }
    }
}

pub enum Palette {
    Black,
    Darker,
    Dark,
    Light,
    Lighter,
    White,
}
impl Palette {
    pub fn srgb(&self) -> Color {
        use Palette::*;
        match self {
            Black => Color::srgb(0.0, 0.0, 0.0),
            Darker => Color::srgb(0.0314, 0.0941, 0.1255),
            Dark => Color::srgb(0.2039, 0.4098, 0.3373),
            Light => Color::srgb(0.5333, 0.7529, 0.4392),
            Lighter => Color::srgb(0.8784, 0.9725, 0.8157),
            White => Color::srgb(1.0, 1.0, 1.0),
        }
    }
}

#[derive(Component)]
pub struct UiBackgroundColor {
    pub normal: Palette,
    pub hovered: Palette,
    pub pressed: Palette,
}
impl Default for UiBackgroundColor {
    fn default() -> Self {
        Self {
            normal: Palette::Light,
            hovered: Palette::Lighter,
            pressed: Palette::Dark,
        }
    }
}

#[derive(Component)]
pub struct UiBorderColor {
    pub normal: Palette,
    pub hovered: Palette,
    pub pressed: Palette,
}
impl Default for UiBorderColor {
    fn default() -> Self {
        Self {
            normal: Palette::Black,
            hovered: Palette::Light,
            pressed: Palette::Darker,
        }
    }
}

#[derive(Component)]
pub struct UiTextColor {
    pub normal: Palette,
    pub hovered: Palette,
    pub pressed: Palette,
}
impl Default for UiTextColor {
    fn default() -> Self {
        Self {
            normal: Palette::Black,
            hovered: Palette::Dark,
            pressed: Palette::Black,
        }
    }
}

fn ui_button_interaction(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut query_text_color: Query<&mut TextColor>,
) {
    for (interaction, mut background_color, mut border_color, children) in &mut interaction_query {
        let mut text_color = query_text_color.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                background_color.0 = UiBackgroundColor::default().pressed.srgb();
                border_color.0 = UiBorderColor::default().pressed.srgb();
                text_color.0 = UiTextColor::default().pressed.srgb();
            }
            Interaction::Hovered => {
                background_color.0 = UiBackgroundColor::default().hovered.srgb();
                border_color.0 = UiBorderColor::default().hovered.srgb();
                text_color.0 = UiTextColor::default().hovered.srgb();
            }
            Interaction::None => {
                background_color.0 = UiBackgroundColor::default().normal.srgb();
                border_color.0 = UiBorderColor::default().normal.srgb();
                text_color.0 = UiTextColor::default().normal.srgb();
            }
        }
    }
}

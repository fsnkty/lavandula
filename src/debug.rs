use bevy::{
    diagnostic::*,
    prelude::*,
    color::palettes::css::GOLD,
};

pub struct DebugHud;
impl Plugin for DebugHud {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, setup_debug_hud)
            .add_systems(Update, update_debug_hud);
    }
}

#[derive(Component)]
enum DebugHudText {
    Fps,
    FrameTime,
    VSync,
}

pub fn setup_debug_hud(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 20.,
                    ..default()
                },
            ),
            TextSection::from_style({
                TextStyle {
                    font_size: 20.,
                    color: GOLD.into(),
                    ..default()
                }
            }),
        ]),
        DebugHudText::Fps,
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "\nFrame Time: ",
                TextStyle {
                    font_size: 20.,
                    ..default()
                },
            ),
            TextSection::from_style({
                TextStyle {
                    font_size: 20.,
                    color: GOLD.into(),
                    ..default()
                }
            }),
        ]),
        DebugHudText::FrameTime,
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "\n\nVSYNC mode: ",
                TextStyle {
                    font_size: 20.,
                    ..default()
                },
            ),
            TextSection::from_style({
                TextStyle {
                    font_size: 20.,
                    color: GOLD.into(),
                    ..default()
                }
            }),
        ]),
        DebugHudText::VSync,
    ));
}

fn update_debug_hud(
    mut textquery: Query<(&mut Text, &DebugHudText)>,
    diagnostics: Res<DiagnosticsStore>,
    windows: Query<&Window>,
) {
    for (mut text, e) in textquery.iter_mut() {
        use DebugHudText::*;
        match e {
            Fps => {
                if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(value) = fps.smoothed() {
                        text.sections[1].value = format!("{value:.0}");
                    }
                }
            }
            FrameTime => {
                if let Some(frame_time_diagnostic) =
                    diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
                {
                    if let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed() {
                        text.sections[1].value = format!("{frame_time_smoothed:.2}");
                    }
                }
            }
            VSync => {
                let Ok(window) = windows.get_single() else {
                    continue;
                };
                text.sections[1].value = format!("{0:?}", window.present_mode)
            }
        }
    }
}

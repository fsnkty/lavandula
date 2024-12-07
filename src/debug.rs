use bevy::{color::palettes::css::GOLD, diagnostic::*, prelude::*};

pub struct DebugHud;
impl Plugin for DebugHud {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
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
    commands.spawn((Text::new("FPS: "),)).with_child((
        TextSpan::default(),
        TextColor(GOLD.into()),
        DebugHudText::Fps,
    ));
    commands.spawn((Text::new("Frame Time: "),)).with_child((
        TextSpan::default(),
        TextColor(GOLD.into()),
        DebugHudText::FrameTime,
    ));
    commands.spawn((Text::new("VSYNC mode: "),)).with_child((
        TextSpan::default(),
        TextColor(GOLD.into()),
        DebugHudText::VSync,
    ));
}

fn update_debug_hud(
    query: Query<(Entity, &DebugHudText)>,
    mut writer: TextUiWriter,
    diagnostics: Res<DiagnosticsStore>,
    windows: Query<&Window>,
) {
    for (entity, e) in query.iter() {
        use DebugHudText::*;
        match e {
            Fps => {
                if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(value) = fps.smoothed() {
                        *writer.text(entity, 1) = format!("{value:.0}");
                    }
                }
            }
            FrameTime => {
                if let Some(frame_time_diagnostic) =
                    diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
                {
                    if let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed() {
                        *writer.text(entity, 1) = format!("{frame_time_smoothed:.2} ms");
                    }
                }
            }
            VSync => {
                if let Ok(window) = windows.get_single() {
                    *writer.text(entity, 1) = format!("{:?}", window.present_mode);
                }
            }
        }
    }
}

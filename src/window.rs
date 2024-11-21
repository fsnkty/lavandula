use bevy::{
    core::FrameCount,
    prelude::*,
    window::PrimaryWindow,
    winit::WinitWindows,
};
use std::io::Cursor;
use winit::window::Icon;


pub struct WindowController;
impl Plugin for WindowController {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            make_visable,
            set_window_icon,
        ));
    }
}

pub fn make_visable(
    mut window: Query<&mut Window>,
    frames: Res<FrameCount>,
) {
    if frames.0 == 3 {
        window.single_mut().visible = true;
    }
}
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!(
        "../build-assets/icon.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
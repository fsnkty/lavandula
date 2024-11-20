use bevy::{
    core::FrameCount,
    prelude::*,
};

pub struct WindowController;
impl Plugin for WindowController {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            make_visable,
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
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use avian2d::{math::*, prelude::*};
use bevy::{prelude::*, window::*, winit::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod debug;
mod window;
mod world;
mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "lavandula".to_string(),
                    resolution: WindowResolution::new(1280., 720.).with_scale_factor_override(1.),
                    // with nothing to draw for the first few frames
                    // delay going visable till 3 frames elapse
                    // handled by the window controller
                    visible: false,
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }),
            WorldInspectorPlugin::new(),
            window::WindowController,
            debug::DebugHud,
            PhysicsPlugins::default().with_length_unit(20.),
            world::WorldManagerPlugin,
            player::PlayerManagerPlugin,
        ))
        .insert_resource(WinitSettings::game())
        // physics resources
        .insert_resource(Gravity(Vector::NEG_Y * 1000.))
        .run();
}
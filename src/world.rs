use avian2d::prelude::*;
use bevy::prelude::*;
pub struct WorldManagerPlugin;
impl Plugin for WorldManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
    }
}

fn setup_world(mut commands: Commands) {
    commands.spawn((
        Name::new("cubefren"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 0.4, 0.7),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_xyz(50.0, -100.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::rectangle(30.0, 30.0),
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(1100.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -175.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(1100.0, 50.0),
        Name::new("floor"),
    ));
}

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
pub struct PlayerManagerPlugin;
impl Plugin for PlayerManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(
                FixedUpdate,
                (
                    update_grounded,
                    player_movement,
                    update_camera,
                    apply_movement_damping,
                )
                    .chain(),
            );
    }
}

// set of player actions
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    #[actionlike(DualAxis)]
    Run,
    Jump,
}
impl Action {
    /// Define the default bindings to the input
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map.insert(Self::Jump, KeyCode::Space);
        input_map.insert_dual_axis(Self::Run, VirtualDPad::wasd());
        input_map
    }
}
// marker as player for entity
#[derive(Component)]
pub struct Player;
/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;
/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(Scalar);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

#[derive(Bundle)]
pub struct PlayerControllerBundle {
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    movement: MovementBundle,
}
impl PlayerControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            rigid_body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_time_of_impact(10.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse, max_slope_angle);
        self
    }
}
/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
    max_slope_angle: MaxSlopeAngle,
}
impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}
impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 8.0, PI * 0.45)
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Name::new("camera"), Camera2d));
    commands
        .spawn((
            Name::new("player"),
            InputManagerBundle::with_map(Action::default_input_map()),
            Mesh2d(meshes.add(Capsule2d::new(12.5, 20.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.2, 0.7, 0.9))),
            Transform::from_xyz(0.0, -100.0, 0.0),
            PlayerControllerBundle::new(Collider::capsule(12.5, 20.0)).with_movement(
                1250.,
                0.92,
                400.,
                (30. as Scalar).to_radians(),
            ),
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            ColliderDensity(2.0),
            GravityScale(1.5),
        ))
        .insert(Player);
}

/// Update the camera position by tracking the player.
fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using interpolation between
    // the camera position and the player position on the x and y axes.
    // Here we use the in-game time, to get the elapsed time (in seconds)
    // since the previous update. This avoids jittery movement when tracking
    // the player.
    camera.translation = camera.translation.lerp(direction, time.delta_secs() * 8.);
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn player_movement(
    time: Res<Time>,
    query: Query<&ActionState<Action>, With<Player>>,
    mut controllers: Query<(
        &MovementAcceleration,
        &JumpImpulse,
        &mut LinearVelocity,
        Has<Grounded>,
    )>,
) {
    let delta_time = time.delta_secs_f64().adjust_precision();
    let action_state = query.single();
    for (movement_acceleration, jump_impulse, mut linear_velocity, is_grounded) in &mut controllers
    {
        if action_state.axis_pair(&Action::Run) != Vec2::ZERO {
            let direction = action_state.axis_pair(&Action::Run).x;
            linear_velocity.x += direction * movement_acceleration.0 * delta_time;
        }
        if action_state.just_pressed(&Action::Jump) && is_grounded {
            linear_velocity.y = jump_impulse.0;
        }
    }
}

fn update_grounded(
    mut commands: Commands,
    mut query: Query<(Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>), With<Player>>,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_to(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

/// Slows down movement in the X direction.
fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
    }
}

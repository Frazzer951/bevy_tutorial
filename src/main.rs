use bevy::{
    core::FixedTimestep,
    math::{const_vec2, Vec3Swizzles},
    prelude::*,
};

const TIME_STEP: f32 = 1.0 / 60.0;
const BOUNDS: Vec2 = const_vec2!([1200.0, 640.0]);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_movement_system), //.with_system(snap_to_player_system)
                                                      //.with_system(rotate_to_player_system)
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

/// Player Component
#[derive(Component)]
struct Player {
    /// linear speed in meters per second
    movement_speed: f32,
    /// rotation speed in radians per second
    rotation_speed: f32,
}

/// snap to palyer ship behavior
#[derive(Component)]
struct SnapToPlayer;

/// rotate to face plater ship behavior
#[derive(Component)]
struct RotateToPlayer {
    /// rotation speed in radians per second
    rotation_speed: f32,
}

/// Add the game's entities to our world and creates an orthographic camera for 2D rendering.
///
/// The Bevy coordinate system is the same for 2D and 3D, in terms of 2D this means that:
///
/// * X axis goes from left to right (+X points right)
/// * Y axis goes from bottom to top (+Y point up)
/// * Z axis goes from far to near (+Z points towards you, out of the screen)
///
/// The origin is at the center of the screen.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship_handle = asset_server.load("textures/simplespace/ship_C.png");
    let enemy_a_handle = asset_server.load("textures/simplespace/enemy_A.png");
    let enemy_b_handle = asset_server.load("textures/simplespace/enemy_B.png");

    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let horizontal_margin = BOUNDS.x / 4.0;
    let vertical_margin = BOUNDS.y / 4.0;

    // Player controlled ship
    commands
        .spawn_bundle(SpriteBundle {
            texture: ship_handle,
            ..default()
        })
        .insert(Player {
            movement_speed: 500.0,                  // meters per second
            rotation_speed: f32::to_radians(360.0), // degrees per second
        });

    // enemy that snaps to face the plater spawns on the bottom adn left
    commands
        .spawn_bundle(SpriteBundle {
            texture: enemy_a_handle.clone(),
            transform: Transform::from_xyz(0.0 - horizontal_margin, 0.0, 0.0),
            ..default()
        })
        .insert(SnapToPlayer);
    commands
        .spawn_bundle(SpriteBundle {
            texture: enemy_a_handle,
            transform: Transform::from_xyz(0.0, 0.0 - vertical_margin, 0.0),
            ..default()
        })
        .insert(SnapToPlayer);

    // enemy that rotates to face the player enemy spawns on the top and right
    commands
        .spawn_bundle(SpriteBundle {
            texture: enemy_b_handle.clone(),
            transform: Transform::from_xyz(0.0 + horizontal_margin, 0.0, 0.0),
            ..default()
        })
        .insert(RotateToPlayer {
            rotation_speed: f32::to_radians(45.0), // degrees per second
        });
    commands
        .spawn_bundle(SpriteBundle {
            texture: enemy_b_handle,
            transform: Transform::from_xyz(0.0, 0.0 + vertical_margin, 0.0),
            ..default()
        })
        .insert(RotateToPlayer {
            rotation_speed: f32::to_radians(90.0), // degrees per second
        });
}

/// Demonstates applying rotation and movement based on keyboard input
fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (ship, mut transform) = query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 1.0;
    }

    // create the change in rotation around the Z axis (perpendicular to the 2D plane of the screen)
    let rotation_delta = Quat::from_rotation_z(rotation_factor * ship.rotation_speed * TIME_STEP);
    // update the ship rotation with our rotation delta
    transform.rotation *= rotation_delta;

    // get the ship's forward vector by applying the current rotation to the ships initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the ship will move based on direction, the ship's movement speed and delta time
    let movement_distance = movement_factor * ship.movement_speed * TIME_STEP;
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;
    // update the ship translation with our new translation delta
    transform.translation += translation_delta;

    // bound the ship within the invisible level bounds
    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}

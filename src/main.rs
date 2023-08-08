#![allow(clippy::too_many_arguments, clippy::type_complexity)]
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
//-----marker COmponents for easy qeuering----
#[derive(Component)]
struct Player;
#[derive(Component)]
struct Player2;
#[derive(Component)]
struct MyCamera;

#[derive(Component)]
struct PlayerMovement {
    movement_speed: f32,
    rotation_speed: f32,
}
#[derive(Component)]
struct Health {
    health: i32,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EditorPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player,move_player2))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MyCamera));
    //---------player1-------------------
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("tank_blue.png"),
            ..Default::default()
        })
        .insert(PlayerMovement {
            movement_speed: 500.,
            rotation_speed: f32::to_radians(360.0),
        })
        .insert(Player);
    //------------player2----------
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("tank_dark.png"),
            ..Default::default()
        })
        .insert(PlayerMovement {
            movement_speed: 500.,
            rotation_speed: f32::to_radians(360.0),
        })
        .insert(Player2);
    commands.spawn(SpriteBundle {
        texture: asset_server.load("map.png"),
        ..Default::default()
    });
}

fn move_player(
    mut query: Query<(&mut Transform, &mut PlayerMovement), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, ship) = query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::A) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
        movement_factor -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        movement_factor += 1.0;
    }

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    transform.rotate_z(rotation_factor * ship.rotation_speed * time.delta_seconds());

    // get the ship's forward vector by applying the current rotation to the ships initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the ship will move based on direction, the ship's movement speed and delta time
    let movement_distance = movement_factor * ship.movement_speed * time.delta_seconds();
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;
    // update the ship translation with our new translation delta
    transform.translation += translation_delta;
}

fn move_player2(
    mut query: Query<(&mut Transform, &mut PlayerMovement), With<Player2>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, ship) = query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        movement_factor += 1.0;
    }

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    transform.rotate_z(rotation_factor * ship.rotation_speed * time.delta_seconds());

    // get the ship's forward vector by applying the current rotation to the ships initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the ship will move based on direction, the ship's movement speed and delta time
    let movement_distance = movement_factor * ship.movement_speed * time.delta_seconds();
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;
    // update the ship translation with our new translation delta
    transform.translation += translation_delta;
}

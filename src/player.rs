use bevy::prelude::*;
use bevy_third_person_camera::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

fn player_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
    camera_q: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_transform, player_speed) in player_q.iter_mut() {
        let camera = match camera_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        // forward
        if input.pressed(KeyCode::W) {
            direction += camera.forward();
        }

        // back
        if input.pressed(KeyCode::S) {
            direction += camera.back();
        }

        // left
        if input.pressed(KeyCode::A) {
            direction += camera.left();
        }

        // right
        if input.pressed(KeyCode::D) {
            direction += camera.right();
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        player_transform.translation += movement;

        // rotate player to face direction he is currently moving
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y);
        }
    }
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let flashlight = (SpotLightBundle::default(), Name::new("FLashlight"));

    let player = (
        SceneBundle {
            scene: assets.load("Player.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Speed(2.5),
        Player,
        ThirdPersonCameraTarget,
        Name::new("Player"),
    );

    commands.spawn(player).with_children(|parent| {
        parent.spawn(flashlight);
    });
}

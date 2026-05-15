use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

use crate::world::ROOM_HALF_EXTENT;

const CAMERA_HEIGHT: f32 = 1.6;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            speed: 6.5,
            sensitivity: 0.0025,
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, lock_cursor))
            .add_systems(Update, (move_player, look_player, toggle_cursor_lock));
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player::default(),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, CAMERA_HEIGHT, 6.0)
                .looking_at(Vec3::new(0.0, CAMERA_HEIGHT, 0.0), Vec3::Y),
            ..default()
        },
        Name::new("Player"),
    ));
}

fn lock_cursor(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }
}

fn toggle_cursor_lock(keys: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }

    if let Ok(mut window) = windows.get_single_mut() {
        let is_locked = window.cursor.grab_mode == CursorGrabMode::Locked;
        if is_locked {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        } else {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        }
    }
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, player) in &mut player_q {
        let mut axis = Vec2::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            axis.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            axis.y -= 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            axis.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            axis.x += 1.0;
        }

        if axis == Vec2::ZERO {
            continue;
        }

        let axis = axis.normalize();
        let forward = (transform.rotation * -Vec3::Z).normalize();
        let right = (transform.rotation * Vec3::X).normalize();
        let horizontal_forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
        let horizontal_right = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

        let direction = horizontal_forward * axis.y + horizontal_right * axis.x;
        transform.translation += direction * player.speed * time.delta_seconds();
        transform.translation.y = CAMERA_HEIGHT;

        let limit = ROOM_HALF_EXTENT - 0.8;
        transform.translation.x = transform.translation.x.clamp(-limit, limit);
        transform.translation.z = transform.translation.z.clamp(-limit, limit);
    }
}

fn look_player(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut player_q: Query<(&mut Transform, &mut Player)>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };

    if window.cursor.grab_mode != CursorGrabMode::Locked {
        mouse_motion_events.clear();
        return;
    }

    let mut mouse_delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        mouse_delta += event.delta;
    }

    if mouse_delta == Vec2::ZERO {
        return;
    }

    for (mut transform, mut player) in &mut player_q {
        player.yaw -= mouse_delta.x * player.sensitivity;
        player.pitch -= mouse_delta.y * player.sensitivity;
        player.pitch = player.pitch.clamp(-1.45, 1.45);

        transform.rotation = Quat::from_axis_angle(Vec3::Y, player.yaw)
            * Quat::from_axis_angle(Vec3::X, player.pitch);
    }
}

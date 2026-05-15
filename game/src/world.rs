use bevy::math::primitives::Cuboid;
use bevy::prelude::*;

pub const ROOM_HALF_EXTENT: f32 = 9.0;
const WALL_HEIGHT: f32 = 3.0;
const WALL_THICKNESS: f32 = 0.3;

pub struct WorldPlugin;

#[derive(Component)]
pub struct Accelerator;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
    }
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.5, 0.0),
        ..default()
    });

    let room_color = materials.add(Color::srgb(0.06, 0.07, 0.09));
    let accelerator_color = materials.add(Color::srgb(0.2, 0.8, 1.0));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(
            ROOM_HALF_EXTENT * 2.0,
            0.2,
            ROOM_HALF_EXTENT * 2.0,
        ))),
        material: room_color.clone(),
        transform: Transform::from_xyz(0.0, -0.1, 0.0),
        ..default()
    });

    let wall_mesh_x = meshes.add(Mesh::from(Cuboid::new(
        ROOM_HALF_EXTENT * 2.0,
        WALL_HEIGHT,
        WALL_THICKNESS,
    )));
    let wall_mesh_z = meshes.add(Mesh::from(Cuboid::new(
        WALL_THICKNESS,
        WALL_HEIGHT,
        ROOM_HALF_EXTENT * 2.0,
    )));

    for z in [-ROOM_HALF_EXTENT, ROOM_HALF_EXTENT] {
        commands.spawn(PbrBundle {
            mesh: wall_mesh_x.clone(),
            material: room_color.clone(),
            transform: Transform::from_xyz(0.0, WALL_HEIGHT / 2.0, z),
            ..default()
        });
    }
    for x in [-ROOM_HALF_EXTENT, ROOM_HALF_EXTENT] {
        commands.spawn(PbrBundle {
            mesh: wall_mesh_z.clone(),
            material: room_color.clone(),
            transform: Transform::from_xyz(x, WALL_HEIGHT / 2.0, 0.0),
            ..default()
        });
    }

    commands.spawn((
        Accelerator,
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(1.2, 1.2, 1.2))),
            material: accelerator_color,
            transform: Transform::from_xyz(0.0, 0.6, 0.0),
            ..default()
        },
        Name::new("Particle Accelerator"),
    ));
}

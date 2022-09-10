use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::WorldInspectorPlugin;

// Window
const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const PLAYER_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            width: WIDTH,
            height: HEIGHT,
            resizable: false,
            position: bevy::window::WindowPosition::Centered(
                bevy::window::MonitorSelection::Current, // Center on the current monitor
            ),
            mode: bevy::window::WindowMode::Windowed,
            ..Default::default()
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn_camera)
        .add_startup_system(create_players)
        .add_system(update_positions)
        .run();
}

#[derive(Component)]
struct Player {
    position: f32,
}

#[derive(Component)]
struct Velocity {
    value: f32,
}

#[derive(Component)]
struct Ball {
    velocity: Vec2,
    position: Vec2,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn create_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Box::new(0.1, 1.0, 0.0)))
                .into(),
            transform: Transform::default()
                .with_scale(Vec3::splat(128.0))
                .with_translation(Vec3::new(-WIDTH / 2.0 + 20.0, 0.0, 0.0)),
            material: materials.add(PLAYER_COLOR.into()),
            ..default()
        })
        .insert(Player {
            position: HEIGHT / 2.0,
        });
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Box::new(0.1, 1.0, 0.0)))
                .into(),
            transform: Transform::default()
                .with_scale(Vec3::splat(128.0))
                .with_translation(Vec3::new(WIDTH / 2.0 - 20.0, 0.0, 0.0)),
            material: materials.add(PLAYER_COLOR.into()),
            ..default()
        })
        .insert(Player {
            position: HEIGHT / 2.0,
        });
}
fn update_positions(mut query: Query<&Player, &mut Transform>) {
    for (player, mut transform) in &mut query {
        transform.translation.y = position.position;
    }
}

///
/// The scoreboard changes the text values as intended, but is not displayed properly
/// 

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, collide_aabb::{collide, Collision}}, time::FixedTimestep};
use bevy_inspector_egui::WorldInspectorPlugin;

// Window
const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const PLAYER_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const WALL_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
const MIDDLE_LINE_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

// Movement
const TIME_STEP: f32 = 1.0 / 60.0;
const PLAYER_SPEED: f32 = 500.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
const BALL_SPEED: f32 = 400.0;

// Scoreboard
const SCORE_FONT_SIZE: f32 = 100.0;
const SCORE_COLOUR: Color = Color::rgb(1.0, 1.0, 1.0);

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
        .insert_resource(Scoreboard::new())
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                //.with_system(move_player.before(check_for_collisions))
        )
        //.add_system(move_player)
        .add_system(update_scoreboard)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct RespawnWall;

#[derive(Component)]
struct BounceWall;

#[derive(Component)]
struct Score;

#[derive(Component, Default)]
struct Scoreboard {
    p1_score: usize,
    p2_score: usize,
}

impl Scoreboard {
    fn new() -> Scoreboard {
        Scoreboard {
            p1_score: 23,
            p2_score: 0,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {

    commands.spawn_bundle(Camera2dBundle::default());

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
        .insert(Player)
        .insert(Collider);
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
        .insert(Player)
        .insert(Collider);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default()
                .with_scale(Vec3::splat(12.0))
                .with_translation(Vec3::new(0.0, 0.0, 1.0)),
            material: materials.add(PLAYER_COLOR.into()),
            ..default()
        })
        .insert(Ball)
        .insert(Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED));

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(10.0, HEIGHT, 0.0))).into(),
            transform: Transform::default()
                .with_translation(Vec3::new(-WIDTH / 2.0 + 2.0, 0.0, 0.0))
                .with_scale(Vec3::splat(1.0)),
            material: materials.add(WALL_COLOR.into()),
            ..default()
        })
        .insert(BounceWall);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(10.0, HEIGHT, 0.0))).into(),
            transform: Transform::default()
                .with_translation(Vec3::new(WIDTH / 2.0 - 2.0, 0.0, 0.0))
                .with_scale(Vec3::splat(1.0)),
            material: materials.add(WALL_COLOR.into()),
            ..default()
        })
        .insert(BounceWall);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(WIDTH, 10.0, 0.0))).into(),
            transform: Transform::default()
                .with_translation(Vec3::new(0.0, HEIGHT / 2.0 - 2.0, 0.0))
                .with_scale(Vec3::splat(1.0)),
            material: materials.add(WALL_COLOR.into()),
            ..default()
        })
        .insert(RespawnWall);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(WIDTH, 10.0, 0.0))).into(),
            transform: Transform::default()
                .with_translation(Vec3::new(0.0, -HEIGHT / 2.0 + 2.0, 0.0))
                .with_scale(Vec3::splat(1.0)),
            material: materials.add(WALL_COLOR.into()),
            ..default()
        })
        .insert(RespawnWall);

    for height in ((-HEIGHT / 2.0 ) / 8.0) as i32..((HEIGHT / 2.0) / 8.0 ) as i32 {
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default()
                .with_translation(Vec3::new(0.0, (-height as f32) * 8.0, 0.0))
                .with_scale(Vec3::splat(2.0)),
            material: materials.add(MIDDLE_LINE_COLOR.into()),
            ..default() 
        });
    }

    // Scoreboard
    commands
    .spawn_bundle(
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {

            // left score
            parent.spawn_bundle(
                TextBundle::from_sections([
                    TextSection::from_style(TextStyle {
                        font: Default::default(),   // There is no bevy default font, this not working is intended i guess
                        font_size: SCORE_FONT_SIZE,
                        color: SCORE_COLOUR,
                    }),
                ])
                .with_text_alignment(TextAlignment::TOP_CENTER)
                .with_style(Style {
                    align_self: AlignSelf::FlexStart,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        bottom: Val::Px(5.0),
                        right: Val::Px(15.0),
                        ..default()
                    },
                    ..default()
                })
            ).insert(Score);

            //right score
            parent.spawn_bundle(
                TextBundle::from_sections([
                    TextSection::from_style(TextStyle {
                        font: Default::default(),
                        font_size: SCORE_FONT_SIZE,
                        color: SCORE_COLOUR,
                    }),
                ])
                .with_style(Style {
                    align_self: AlignSelf::FlexEnd,
                    ..default()
                })
            ).insert(Score);
        });

}

// fn move_player(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<&mut Transform, With<Player>>,
// ) {
//     let mut paddle_transform = query.single_mut();
//     let mut direction = 0.0;

//     if keyboard_input.pressed(KeyCode::Up) {
//         direction -= 1.0;
//     }

//     if keyboard_input.pressed(KeyCode::Down) {
//         direction += 1.0;
//     }

//     // Calculate the new horizontal paddle position based on player input
//     let new_paddle_position = paddle_transform.translation.y + direction * PLAYER_SPEED * TIME_STEP;

//     // Update the paddle position,
//     // making sure it doesn't cause the paddle to leave the arena
//     let left_bound = -WIDTH / 2.0;
//     let right_bound = WIDTH / 2.0;

//     paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
// }


// fn check_for_collisions(
//     mut commands: Commands,
//     mut scoreboard: ResMut<Scoreboard>,
//     mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
//     collider_query: Query<(Entity, &Transform, Option<&RespawnWall>), With<Collider>>,
//     mut collision_events: EventWriter<CollisionEvent>,
// ) {
//     let (mut ball_velocity, ball_transform) = ball_query.single_mut();
//     let ball_size = ball_transform.scale.truncate();

//     // check collision with walls
//     for (collider_entity, transform, maybe_brick) in &collider_query {
//         let collision = collide(
//             ball_transform.translation,
//             ball_size,
//             transform.translation,
//             transform.scale.truncate(),
//         );
//         if let Some(collision) = collision {
//             // Sends a collision event so that other systems can react to the collision
//             collision_events.send_default();

//             // Bricks should be despawned and increment the scoreboard on collision
//             if maybe_brick.is_some() {
//                 scoreboard.player1 += 1;
//                 commands.entity(collider_entity).despawn();
//             }

//             // reflect the ball when it collides
//             let mut reflect_x = false;
//             let mut reflect_y = false;

//             // only reflect if the ball's velocity is going in the opposite direction of the
//             // collision
//             match collision {
//                 Collision::Left => reflect_x = ball_velocity.x > 0.0,
//                 Collision::Right => reflect_x = ball_velocity.x < 0.0,
//                 Collision::Top => reflect_y = ball_velocity.y < 0.0,
//                 Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
//                 Collision::Inside => { /* do nothing */ }
//             }

//             // reflect velocity on the x-axis if we hit something on the x-axis
//             if reflect_x {
//                 ball_velocity.x = -ball_velocity.x;
//             }

//             // reflect velocity on the y-axis if we hit something on the y-axis
//             if reflect_y {
//                 ball_velocity.y = -ball_velocity.y;
//             }
//         }
//     }
// }

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut query = query.iter_mut();

    query.next().unwrap()
        .sections[0].value = scoreboard.p1_score.to_string();

    query.next().unwrap()
        .sections[0].value = scoreboard.p2_score.to_string();
}

///
/// The scoreboard changes the text values as intended, but is not displayed properly
///
use bevy::{
    prelude::*,
    sprite::{
        collide_aabb::{collide, Collision},
        MaterialMesh2dBundle,
    },
    text::Text2dBounds,
    time::FixedTimestep,
};

// Window
const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const PLAYER_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const WALL_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
const MIDDLE_LINE_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

// Ball
const BALL_COLOUR: Color = Color::rgb(1.0, 1.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::splat(12.0);
const BALL_START_POS: Vec3 = Vec3::splat(0.0);

// Movement
const TIME_STEP: f32 = 1.0 / 60.0;
const PLAYER_SPEED: f32 = 500.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.4, -0.1);
const BALL_SPEED: f32 = 400.0;

// Scoreboard
const SCORE_FONT_SIZE: f32 = 100.0;
const SCORE_COLOUR: Color = Color::rgb(1.0, 1.0, 1.0);
const SCORE_FONT: &str = "fonts/MinecraftMono.otf";

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
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_for_collisions)
                .with_system(move_player.before(check_for_collisions))
                .with_system(apply_velocity.before(check_for_collisions)),
        )
        .add_system(update_scoreboard)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct RespawnWallLeft;

#[derive(Component)]
struct RespawnWallRight;

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
            p1_score: 0,
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
        // Ball(in)
        .spawn()
        .insert(Ball)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: BALL_SIZE,
                translation: BALL_START_POS,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOUR,
                ..default()
            },
            ..default()
        })
        .insert(Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED));

    // Left player (player 1)
    commands
        .spawn()
        .insert(Player)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-WIDTH/2.0 + 20.0, 0.0, 0.0),
                scale: Vec3::new(12.8, 128.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Collider);

    // Right player (player 2)
    commands
        .spawn()
        .insert(Player)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(WIDTH/2.0 - 20.0, 0.0, 0.0),
                scale: Vec3::new(12.8, 128.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Collider);

    // Left wall
    commands
        .spawn()
        .insert(RespawnWallLeft)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-WIDTH / 2.0, 0.0, 0.0),
                scale: Vec3::new(10.0, HEIGHT, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR.into(),
                ..default()
            },
            ..default()
        })
        .insert(Collider);

    // Right Wall
    commands
        .spawn()
        .insert(RespawnWallRight)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(WIDTH / 2.0, 0.0, 0.0),
                scale: Vec3::new(10.0, HEIGHT, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR.into(),
                ..default()
            },
            ..default()
        })
        .insert(Collider);

    // Top wall
    commands
        .spawn()
        .insert(BounceWall)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, HEIGHT / 2.0, 0.0),
                scale: Vec3::new(WIDTH, 10.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR.into(),
                ..default()
            },
            ..default()
        })
        .insert(Collider);

    // Bottom Wall
    commands
        .spawn()
        .insert(BounceWall)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -HEIGHT / 2.0, 0.0),
                scale: Vec3::new(WIDTH, 10.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR.into(),
                ..default()
            },
            ..default()
        })
        .insert(Collider);

    // Make the middle line by spawning a bunch of small quads
    for height in ((-HEIGHT / 2.0) / 8.0) as i32..((HEIGHT / 2.0) / 8.0) as i32 {
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default()
                .with_translation(Vec3::new(0.0, (-height as f32) * 8.0, 0.0))
                .with_scale(Vec3::splat(2.0)),
            material: materials.add(MIDDLE_LINE_COLOR.into()),
            ..default()
        });
    }

    // Style of the text, using font as specified by the constant
    let text_style = TextStyle {
        font: asset_server.load(SCORE_FONT),
        font_size: SCORE_FONT_SIZE,
        color: SCORE_COLOUR.into(),
    };

    // Scoreboard
    commands
        // Left
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("0", text_style.clone()).with_alignment(TextAlignment::CENTER),
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(100.0, 100.0),
                ..default()
            },
            transform: Transform::from_xyz(-WIDTH / 4.0, HEIGHT / 2.0 - 100.0, 1.0),
            ..default()
        })
        .insert(Score);

    commands
        // Right
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("0", text_style.clone()).with_alignment(TextAlignment::CENTER),
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(100.0, 100.0),
                ..default()
            },
            transform: Transform::from_xyz(WIDTH / 4.0, HEIGHT / 2.0 - 100.0, 1.0),
            ..default()
        })
        .insert(Score);
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut direction_player1 = 0.0;
    let mut direction_player2 = 0.0;

    if keyboard_input.pressed(KeyCode::W) {
        direction_player1 += 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction_player1 -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::O) {
        direction_player2 += 1.0;
    }

    if keyboard_input.pressed(KeyCode::L) {
        direction_player2 -= 1.0;
    }

    // If the current player is player 1, we add to that players velocity, if it is player 2 we add to that
    query
        .iter_mut()
        .enumerate()
        .for_each(|(index, mut transform)| {
            let left_bound = -HEIGHT / 2.0 + 80.0;
            let right_bound = HEIGHT / 2.0 - 80.0;

            if index == 0 {
                let new_paddle_position =
                    transform.translation.y + direction_player1 * PLAYER_SPEED * TIME_STEP;

                transform.translation.y = new_paddle_position.clamp(left_bound, right_bound);
            } else {
                let new_paddle_position =
                    transform.translation.y + direction_player2 * PLAYER_SPEED * TIME_STEP;

                transform.translation.y = new_paddle_position.clamp(left_bound, right_bound);
            }
        });
}

fn check_for_collisions(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Velocity, &mut Transform), With<Ball>>,
    collider_query: Query<(&Collider, &Transform, Option<&RespawnWallLeft>, Option<&RespawnWallRight>), Without<Ball>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, mut ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    // check collision with walls
    for (collider_entity, collider_transform, maybe_left, maybe_right) in &collider_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            collider_transform.translation,
            collider_transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            // Bricks should be despawned and increment the scoreboard on collision
            if maybe_left.is_some() {
                scoreboard.p2_score += 1;
                ball_transform.translation = Vec3::splat(0.0);
            }

            if maybe_right.is_some() {
                scoreboard.p1_score += 1;
                ball_transform.translation = Vec3::splat(0.0);
            }

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}

// Update all of the text elements to the corresponding score 
fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut query = query.iter_mut();

    query.next().unwrap().sections[0].value = scoreboard.p1_score.to_string();

    query.next().unwrap().sections[0].value = scoreboard.p2_score.to_string();
}

// Add velocity to the ball
fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

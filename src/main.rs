use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const BALL_SIZE: f32 = 5.;
const BALL_SPEED: f32 = 5.;
const PADDLE_SPEED: f32 = 1.;
const PADDLE_HEIGHT: f32 = 10.;
const PADDLE_WIDTH: f32 = 50.;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Shape(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Paddle;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    position: Position,
    shape: Shape,
    velocity: Velocity
}
 #[derive(Bundle)]
 struct PaddleBundle {
     paddle: Paddle,
     position: Position
 }

impl BallBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            ball: Ball,
            position: Position(Vec2::new(0., 0.)),
            shape: Shape(Vec2::splat(BALL_SIZE)),
            velocity: Velocity(Vec2::new(x, y))
        }
    }
}

impl PaddleBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            paddle: Paddle,
            position: Position(Vec2::new(x, y))
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            spawn_ball,
            spawn_camera,
            spawn_paddles
        ))
        .add_systems(Update, move_ball)
        .run();
}

fn project_positions(mut ball: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut ball {
        transform.translation = position.0.extend(0.);
    }
}

fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0 * BALL_SPEED;
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball...");
    let mesh = Mesh::from(shape::Circle::new(5.));
    let material = ColorMaterial::from(Color::rgb(1., 0., 0.));

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    commands.spawn((
        BallBundle::new(1., 1.),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
    ));
}

fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning paddles...");

    let mesh = Mesh::from(shape::Quad::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)));
    let material = ColorMaterial::from(Color::rgb(0., 1., 0.));

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    commands.spawn((
        PaddleBundle::new(20., -25.0),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}
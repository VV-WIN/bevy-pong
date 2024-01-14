use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision}, // TODO: Replace with Rapier 2D Physics
    sprite::MaterialMesh2dBundle,
};

const BALL_WIDTH: f32 = 10.;
const BALL_SPEED: f32 = 5.; 
const PADDLE_SPEED: f32 = 1.; // Will come in handy when we start to move the paddles
const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 50.;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Shape(Vec2);

#[derive(Component)]
struct Velocity(Vec2);


#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    shape: Shape,
    velocity: Velocity,
    position: Position
}

impl BallBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            ball: Ball,
            shape: Shape(Vec2::new(BALL_WIDTH, BALL_WIDTH)),
            velocity: Velocity(Vec2::new(x, y)),
            position: Position(Vec2::new(0., 0.))
        }
    }
}

#[derive(Component)]
struct Paddle;

#[derive(Bundle)]
struct PaddleBundle {
    paddle: Paddle,
    shape: Shape,
    velocity: Velocity,
    position: Position
}

impl PaddleBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            paddle: Paddle,
            shape: Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            velocity: Velocity(Vec2::new(0., 0.)),
            position: Position(Vec2::new(x, y))
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            spawn_ball, 
            spawn_paddles,
            spawn_camera, 
        ))
        .add_systems(Update, (
            move_ball,
            project_positions.after(move_ball),
            handle_collisions.after(move_ball),
        ))
        .run();
}

fn handle_collisions(
    mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    // We can collide with anything else that has a shape and position that is
    // not itself a ball
    other_things: Query<(&Position, &Shape), Without<Ball>>,
) {
    if let Ok((mut ball_velocity, ball_position, ball_shape)) = ball.get_single_mut() {
        for (position, shape) in &other_things {
            if let Some(collision) = collide(
                ball_position.0.extend(0.), // position_a (Vec3)
                ball_shape.0,               // size_a (Vec2)
                position.0.extend(0.),      // position_b (Vec3)
                shape.0,                    // size_b (Vec2)
            ) {
                match collision {
                    Collision::Left => {
                        ball_velocity.0.x *= -1.;
                    }
                    Collision::Right => {
                        ball_velocity.0.x *= -1.;
                    }
                    Collision::Top => {
                        ball_velocity.0.y *= -1.;
                    }
                    Collision::Bottom => {
                        ball_velocity.0.y *= -1.;
                    }
                    Collision::Inside => {
                        // Do nothing
                    }
                }
            }
        }
    }
}

fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0 * BALL_SPEED;
    }
}

fn project_positions(mut ball: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut ball {
        transform.translation = position.0.extend(0.);
    }
}


fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball...");

    let mesh = Mesh::from(shape::Circle::new(BALL_WIDTH / 2.0));
    let material = ColorMaterial::from(Color::rgb(1., 0., 0.));

    // Now our mesh shape is derived from the `Shape` we made as a new component
    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    commands.spawn((
        BallBundle::new(1., 0.),
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
    window: Query<&Window>,
) {
    println!("Spawning paddles...");

    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();
        // right and left of the screen with a bit of padding
        let padding = 50.;
        let right_paddle_x = window_width / 2. - padding;
        let left_paddle_x = -window_width / 2. + padding;

        let mesh = Mesh::from(shape::Quad::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)));
        let mesh_handle = meshes.add(mesh);

        let right_paddle_material = ColorMaterial::from(Color::rgb(0., 1., 0.));
        let left_paddle_material = ColorMaterial::from(Color::rgb(0., 0., 1.));

        commands.spawn((
            PaddleBundle::new(right_paddle_x, 0.),
            MaterialMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                material: materials.add(right_paddle_material),
                ..default()
            },
        ));

        commands.spawn((
            PaddleBundle::new(left_paddle_x, 0.),
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: materials.add(left_paddle_material),
                ..default()
            },
        ));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}   
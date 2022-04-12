use bevy::{
    math::vec2,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

const BALL_SPRITE: &str = "ball.png";

const W_WIDTH: f32 = 1380.0;
const W_HEIGHT: f32 = 720.0;
const HALF_W_WIDTH: f32 = W_WIDTH / 2.0;
const HALF_W_HEIGHT: f32 = W_HEIGHT / 2.0;

#[derive(Component)]
struct Ball {
    velocity: Vec3,
}

#[derive(Component)]
struct Player {
    binds: KeyBinds,
    velocity: Vec3,
}
pub struct PlayerBundle {
    binds: KeyBinds,
}

#[derive(Component)]
enum Collider {
    Solid,
    Scoreable,
}

struct KeyBinds {
    up: KeyCode,
    down: KeyCode,
}

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

#[derive(Default)]
struct Manifold {
    penetration: f32,
    normal: Vec2,
    overlap: Vec2,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Rust Pong".to_string(),
            width: W_WIDTH,
            height: W_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup.system())
        .add_system(ball_movement_system.system())
        .add_system(ball_collision_system.system())
        .add_system(player_input_system.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    // position window
    let window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(0, 0));

    // camera
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    // ball
    ball_setup(
        &mut commands,
        asset_server,
        vec2(48.0, 48.0),
        vec2(0.0, 0.0),
    );
    // player 1
    player_setup(
        &mut commands,
        vec2(25.0, 100.0),
        vec2(-HALF_W_WIDTH + 50.0, 0.0),
        KeyCode::W,
        KeyCode::S,
    );

    // player 2
    player_setup(
        &mut commands,
        vec2(25.0, 100.0),
        vec2(HALF_W_WIDTH - 50.0, 0.0),
        KeyCode::O,
        KeyCode::L,
    );

    // top walls
    plataform_setup(
        &mut commands,
        vec2(W_WIDTH, 10.0),
        vec2(0.0, HALF_W_HEIGHT - 5.0),
    );
    // botton wall
    plataform_setup(
        &mut commands,
        vec2(W_WIDTH, 10.0),
        vec2(0.0, -HALF_W_HEIGHT + 5.0),
    );
    // left wall
    plataform_setup(
        &mut commands,
        vec2(10.0, W_HEIGHT),
        vec2(-HALF_W_WIDTH + 5.0, 0.0),
    );
    // right wall
    plataform_setup(
        &mut commands,
        vec2(10.0, W_HEIGHT),
        vec2(HALF_W_WIDTH - 5.0, 0.0),
    );
}

fn ball_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    size: Vec2,
    translation: Vec2,
) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load(BALL_SPRITE),
            transform: Transform::from_translation(Vec3::new(translation.x, translation.y, 0.0)),
            sprite: Sprite {
                custom_size: Some(size),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });
}

fn ball_movement_system(time: Res<Time>, mut query: Query<(&mut Ball, &mut Transform)>) {
    let delta = time.delta_seconds();

    for (mut ball, mut transform) in query.iter_mut() {
        transform.translation += ball.velocity * delta;
    }
}

fn ball_collision_system(
    mut ball_query: Query<(&mut Ball, &mut Sprite, &mut Transform)>,
    other_query: Query<(&Collider, &Transform, &Sprite, Without<Ball>)>,
) {
    for (mut ball, mut ball_sprite, mut ball_transform) in ball_query.iter_mut() {
        let ball_size = ball_sprite.custom_size.unwrap();
        let velocity = &mut ball.velocity;

        for (collider, other_transform, other_sprite, _) in other_query.iter() {
            let collision = collide(
                ball_transform.translation,
                ball_size,
                other_transform.translation,
                other_sprite.custom_size.unwrap(),
            );
            if let Some(collision) = collision {
                ball_sprite.color = Color::rgb(1.0, 0.0, 0.0);

                match collision {
                    Collision::Left => {
                        if velocity.x > 0.0 {
                            velocity.x = -velocity.x;
                            ball_transform.translation.x = other_transform.translation.x
                                - other_sprite.custom_size.unwrap().x / 2.0
                                - ball_size.x / 2.0;
                        }
                    }
                    Collision::Right => {
                        if velocity.x < 0.0 {
                            velocity.x = -velocity.x;
                            ball_transform.translation.x = other_transform.translation.x
                                + other_sprite.custom_size.unwrap().x / 2.0
                                + ball_size.x / 2.0;
                        }
                    }
                    Collision::Top => {
                        if velocity.y < 0.0 {
                            velocity.y = -velocity.y;
                            ball_transform.translation.y = other_transform.translation.y
                                + other_sprite.custom_size.unwrap().y / 2.0
                                + ball_size.y / 2.0;
                        }
                    }
                    Collision::Bottom => {
                        if velocity.y > 0.0 {
                            velocity.y = -velocity.y;
                            ball_transform.translation.y = other_transform.translation.y
                                - other_sprite.custom_size.unwrap().y / 2.0
                                - ball_size.y / 2.0;
                        }
                    }
                }

                break;
            } else {
                ball_sprite.color = Color::rgb(1.0, 1.0, 1.0);
            }
        }
    }
}

fn player_setup(
    commands: &mut Commands,
    size: Vec2,
    translation: Vec2,
    up_keycode: KeyCode,
    down_keycode: KeyCode,
) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(translation.x, translation.y, 0.0)),
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(size.x, size.y)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            binds: KeyBinds {
                up: up_keycode,
                down: down_keycode,
            },
            velocity: Vec3::new(0.0, 400.0, 0.0),
        })
        .insert(Collider::Solid);
}

fn player_input_system(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let delta = time.delta_seconds();
    for (player, mut transform) in query.iter_mut() {
        if input.pressed(player.binds.up) {
            transform.translation += player.velocity * delta;
        } else if input.pressed(player.binds.down) {
            transform.translation -= player.velocity * delta;
        }
    }
}

fn plataform_setup(commands: &mut Commands, size: Vec2, translation: Vec2) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(translation.x, translation.y, 0.0)),
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(size.x, size.y)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Solid);
}

fn collision_aabb(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> (bool, Manifold) {
    let vector: Vec2 = b_pos - a_pos;
    let mut manifold: Manifold = Default::default();
    let mut collision = false;

    // Calculate half extents along x axis for each object
    let ax_ext = a_size.x / 2.0;
    let bx_ext = b_size.x / 2.0;

    // Calculate overlap on x axis
    manifold.overlap.x = ax_ext + bx_ext - vector.x.abs();

    // SAT test on x axis
    if manifold.overlap.x > 0.0 {
        // Calculate half extents along x axis for each object
        let ay_ext = a_size.y / 2.0;
        let by_ext = b_size.y / 2.0;

        // Calculate overlap on x axis
        manifold.overlap.x = ay_ext + by_ext - vector.x.abs();

        // SAT test on y axis
        if manifold.overlap.y > 0.0 {
            // Find out which axis is axis of least penetration
            if manifold.overlap.x < manifold.overlap.y {
                if vector.x < 0.0 {
                    manifold.normal = Vec2::new(-1.0, 0.0);
                } else {
                    manifold.normal = Vec2::new(1.0, 0.0);
                }

                // Calculate the penetration depth
                manifold.penetration = manifold.overlap.x;
                collision = true;
            } else {
                // Point toward B knowing that n points from A to B
                if vector.y < 0.0 {
                    manifold.normal = Vec2::new(0.0, -1.0);
                } else {
                    manifold.normal = Vec2::new(0.0, 1.0);
                }

                // Calculate the penetration depth
                manifold.penetration = manifold.overlap.y;
                collision = true;
            }
        }
    }

    (collision, manifold)
}

fn my_cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_query: Query<(&mut Ball, &mut Sprite, &mut Transform)>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();
    let (ball, ball_sprite, mut ball_transform) = q_query.single_mut();

    // get the window that the camera is displaying to
    let wnd = wnds.get(camera.window).unwrap();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        ball_transform.translation.x = world_pos.x;
        ball_transform.translation.y = world_pos.y;
    }
}

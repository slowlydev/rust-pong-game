use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy_prototype_lyon::prelude::*;

#[derive(Component)]
struct Slider;

#[derive(Component)]
struct PlayerSlider;

#[derive(Component)]
struct EnemySlider;

#[derive(Component)]
struct Ball {
    velocity: Vec3,
    size: Vec2,
}

#[derive(Component)]
enum Collider {
    EnemyScore,
    PlayerScore,
    Solid,
}

fn main() {
    App::new()
        // add resources
        .insert_resource(WindowDescriptor {
            title: "pong-game".to_string(),
            width: 1300.0,
            height: 620.0,
            resizable: false,
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        // add bevy
        .add_plugins(DefaultPlugins)
        // add additional plugins
        .add_plugin(ShapePlugin)
        // add startup systems
        .add_startup_system(setup)
        // add looping systems
        .add_system(slider_move)
        .add_system(ball_move)
        .add_system(enemy_move)
        .add_system(collsion_system)
        .run();
}

fn setup(mut commands: Commands) {
    // spawn 2d cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // spawn player slider
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10.0, 80.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-600.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(PlayerSlider)
        .insert(Slider)
        .insert(Collider::Solid);

    // spawn enemy slider
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10.0, 80.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(600.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(EnemySlider)
        .insert(Slider)
        .insert(Collider::Solid);

    // spawn ball
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Circle {
                radius: 15.,
                ..shapes::Circle::default()
            },
            DrawMode::Fill(FillMode::color(Color::PINK)),
            Transform::default(),
        ))
        .insert(Ball {
            velocity: 700.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
            size: Vec2::new(20., 20.),
        });

    // spawn walls
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(1300.0, 5.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., 310., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Collider::Solid);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(1300.0, 5.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., -310., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Collider::Solid);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::new(5.0, 620.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(650., 0., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Collider::EnemyScore);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::PURPLE,
                custom_size: Some(Vec2::new(5.0, 620.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-650., 0., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Collider::PlayerScore);
}

fn slider_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<
        &mut Transform,
        (
            With<Slider>,
            With<PlayerSlider>,
            Without<EnemySlider>,
            Without<Ball>,
        ),
    >, // selects Sprites with PlayerSlider struct
) {
    for mut transform in query.iter_mut() {
        // iterates through the selcted sprites
        if keys.any_pressed([KeyCode::W, KeyCode::Up]) {
            // add to x if up is pressed
            if transform.translation.y < 310.0 {
                transform.translation.y += 350.0 * time.delta_seconds();
            }
        }
        if keys.any_pressed([KeyCode::S, KeyCode::Down]) {
            // subtract from x if down is pressed
            if transform.translation.y > -310.0 {
                transform.translation.y -= 350.0 * time.delta_seconds();
            }
        }
    }
}

fn enemy_move(
    time: Res<Time>,
    mut enemy_query: Query<
        &mut Transform,
        (With<Sprite>, With<EnemySlider>, Without<PlayerSlider>),
    >,
    ball_query: Query<
        &Transform,
        (
            With<Ball>,
            Without<Slider>,
            Without<PlayerSlider>,
            Without<EnemySlider>,
        ),
    >,
) {
    for mut enemy_transform in enemy_query.iter_mut() {
        for ball_transform in ball_query.iter() {
            if ball_transform.translation.y > enemy_transform.translation.y {
                // if ball is above enemy platform
                if enemy_transform.translation.y < 310.0 {
                    // if enemy can go further up
                    enemy_transform.translation.y += 350.0 * time.delta_seconds();
                }
            }
            if ball_transform.translation.y < enemy_transform.translation.y {
                // if ball is beneth enemy platform
                if enemy_transform.translation.y > -310.0 {
                    // if enemy can go further down
                    enemy_transform.translation.y -= 350.0 * time.delta_seconds();
                }
            }
        }
    }
}

fn ball_move(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
    for (ball, mut transform) in query.iter_mut() {
        transform.translation += ball.velocity * time.delta_seconds();
    }
}

fn collsion_system(
    collision_query: Query<(&Transform, &Sprite, &Collider), Without<Ball>>,
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
) {
    for (mut ball, mut ball_transform) in ball_query.iter_mut() {
        let ball_size = ball.size;
        let velocity = &mut ball.velocity;

        for (collision_transform, collision_sprite, collider) in collision_query.iter() {
            let collision_size = collision_sprite.custom_size.unwrap();

            let collision = collide(
                ball_transform.translation,
                ball_size,
                collision_transform.translation,
                collision_size,
            );

            if let Some(collision) = collision {
                match collider {
                    Collider::EnemyScore => {
                        println!("player scored");
                    }
                    Collider::PlayerScore => {
                        println!("enemy scored");
                    }
                    Collider::Solid => {}
                }

                match collision {
                    Collision::Left => {
                        if velocity.x > 0.0 {
                            velocity.x = -velocity.x;
                            ball_transform.translation.x = collision_transform.translation.x
                                - collision_sprite.custom_size.unwrap().x / 2.0
                                - ball_size.x / 2.0;
                        }
                    }
                    Collision::Right => {
                        if velocity.x < 0.0 {
                            velocity.x = -velocity.x;
                            ball_transform.translation.x = collision_transform.translation.x
                                + collision_sprite.custom_size.unwrap().x / 2.0
                                + ball_size.x / 2.0;
                        }
                    }
                    Collision::Top => {
                        if velocity.y < 0.0 {
                            velocity.y = -velocity.y;
                            ball_transform.translation.y = collision_transform.translation.y
                                + collision_sprite.custom_size.unwrap().y / 2.0
                                + ball_size.y / 2.0;
                        }
                    }
                    Collision::Bottom => {
                        if velocity.y > 0.0 {
                            velocity.y = -velocity.y;
                            ball_transform.translation.y = collision_transform.translation.y
                                - collision_sprite.custom_size.unwrap().y / 2.0
                                - ball_size.y / 2.0;
                        }
                    }
                    Collision::Inside => {
                        if velocity.y > 0.0 {
                            println!("bounce");
                        }
                    }
                }

                break;
            }
        }
    }
}

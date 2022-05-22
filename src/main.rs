use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
// use bevy::sprite::collide_aabb::{collide, Collision};
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Component)]
struct PlayerSlider;

#[derive(Component)]
struct EnemySlider;

#[derive(Component)]
struct Ball;

fn main() {
    App::new()
        // add resources
        .insert_resource(WindowDescriptor {
            title: "pong-game".to_string(),
            resizable: false,
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        // add bevy
        .add_plugins(DefaultPlugins)
        // add additional plugins
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ShapePlugin)
        // add startup systems
        .add_startup_system(setup)
        // add looping systems
        .add_system(slider_move)
        .add_system(ball_move)
        .add_system(enemy_move)
        .run();
}

fn setup(mut commands: Commands) {
    let circle_shape = shapes::Circle {
        radius: 15.,
        ..shapes::Circle::default()
    };

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
        .insert(PlayerSlider);

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
        .insert(EnemySlider);

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &circle_shape,
            DrawMode::Fill(FillMode::color(Color::PINK)),
            Transform::default(),
        ))
        .insert(Ball);
}

fn slider_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Sprite>, With<PlayerSlider>, Without<EnemySlider>)>, // selects Sprites with PlayerSlider struct
) {
    for mut transform in query.iter_mut() {
        // iterates through the selcted sprites
        if keys.any_pressed([KeyCode::W, KeyCode::Up]) {
            // add to x if up is pressed
            if transform.translation.y < 310.0 {
                transform.translation.y += 450.0 * time.delta_seconds();
            }
        }
        if keys.any_pressed([KeyCode::S, KeyCode::Down]) {
            // subtract from x if down is pressed
            if transform.translation.y > -310.0 {
                transform.translation.y -= 450.0 * time.delta_seconds();
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
    ball_query: Query<&Transform, (With<Ball>, Without<PlayerSlider>, Without<EnemySlider>)>,
) {
    for mut enemy_transform in enemy_query.iter_mut() {
        for ball_transform in ball_query.iter() {
            if ball_transform.translation.y < enemy_transform.translation.y {
                enemy_transform.translation.y -= 450.0 * time.delta_seconds();
            }

            if ball_transform.translation.y > enemy_transform.translation.y {
                enemy_transform.translation.y += 450.0 * time.delta_seconds();
            }
        }
    }
}

fn ball_move(time: Res<Time>, mut query: Query<&mut Transform, With<Ball>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x += 350. * time.delta_seconds();
        transform.translation.y += 50. * time.delta_seconds();
    }
}

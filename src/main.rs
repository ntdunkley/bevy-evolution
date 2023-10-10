use bevy::prelude::Commands;
use bevy::prelude::*;
use bevy::DefaultPlugins;

const WIDTH: f32 = 600.0;
const HEIGHT: f32 = 300.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                update_movable_target_pos,
                choose_target_for_predator,
                update_predator_direction,
                move_movables,
            ),
        )
        .run();
}

#[derive(Component)]
struct Movable {
    direction: Vec3,
    speed: f32,
}

#[derive(Component)]
struct Wanderer {
    target_pos: Vec3,
}

#[derive(Component)]
struct Predator {
    target: Option<Entity>,
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    for _ in 0..15 {
        let target_pos = get_random_pos_within(WIDTH, HEIGHT);
        let spawn_pos = get_random_pos_within(WIDTH, HEIGHT);
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(25.0, 25.0)),
                    ..default()
                },
                transform: Transform {
                    translation: spawn_pos,
                    ..default()
                },
                ..default()
            })
            .insert(Movable {
                direction: (target_pos - spawn_pos).normalize(),
                speed: 100.0,
            })
            .insert(Wanderer { target_pos });
    }
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.75, 0.25, 0.25),
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            ..default()
        })
        .insert(Predator { target: None })
        .insert(Movable {
            direction: Vec3::default(),
            speed: 125.0,
        });
}

fn update_movable_target_pos(mut query: Query<(&mut Movable, &mut Wanderer, &Transform)>) {
    for (mut movable, mut wanderer, movable_pos) in query.iter_mut() {
        if wanderer.target_pos.distance(movable_pos.translation) < 5.0 {
            let target_pos = get_random_pos_within(WIDTH, HEIGHT);
            wanderer.target_pos = target_pos;
            movable.direction = (target_pos - movable_pos.translation).normalize();
        }
    }
}

fn move_movables(mut query: Query<(&Movable, &mut Transform)>, time: Res<Time>) {
    for (movable, mut movable_pos) in query.iter_mut() {
        movable_pos.translation += movable.direction * movable.speed * time.delta_seconds();
    }
}

fn update_predator_direction(
    mut commands: Commands,
    movable_query: Query<&Transform, (With<Movable>, Without<Predator>)>,
    mut predator_query: Query<(&mut Predator, &mut Movable, &Transform), With<Predator>>,
) {
    let (mut predator, mut movable, predator_pos) = predator_query.single_mut();
    if let Some(movable_entity) = predator.target {
        if let Ok(target_pos) = movable_query.get(movable_entity) {
            let vector = target_pos.translation - predator_pos.translation;
            if vector.length() < 25.0 {
                commands.entity(movable_entity).despawn();
                predator.target = None;
            } else {
                movable.direction = vector.normalize();
            }
        }
    }
}

fn choose_target_for_predator(
    movable_query: Query<Entity, Without<Predator>>,
    mut predator_query: Query<&mut Predator>,
) {
    let mut predator = predator_query.single_mut();
    if predator.target.is_none() {
        let movable_list = movable_query.iter().collect::<Vec<Entity>>();
        predator.target = get_next_target_for_predator(&movable_list);
    }
}

fn get_next_target_for_predator(movable_list: &Vec<Entity>) -> Option<Entity> {
    if movable_list.is_empty() {
        None
    } else {
        movable_list
            .get(fastrand::usize(0..movable_list.len()))
            .cloned()
    }
}

fn get_random_pos_within(width: f32, height: f32) -> Vec3 {
    Vec3::new(
        (fastrand::f32() * (width * 2.0)) - width,
        (fastrand::f32() * (height * 2.0)) - height,
        0.0,
    )
}

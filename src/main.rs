use bevy::prelude::Commands;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use std::time::Duration;

const WIDTH: f32 = 600.0;
const HEIGHT: f32 = 300.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                update_wanderers,
                update_predators,
                choose_target_for_predator.before(update_predators),
                move_movables,
                spawn_wanderers,
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
    offspring_timer: Timer,
}

#[derive(Component)]
struct Predator {
    target: Option<Entity>,
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    for _ in 0..15 {
        let spawn_pos = get_random_pos_within(WIDTH, HEIGHT);
        let target_pos = get_random_pos_within(WIDTH, HEIGHT);
        let offspring_timer_count = fastrand::u64(10..=20);
        spawn_wanderer(&mut commands, spawn_pos, target_pos, offspring_timer_count);
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
            speed: 150.0,
        });
}

fn spawn_wanderers(
    mut commands: Commands,
    mut query: Query<(&mut Wanderer, &Transform)>,
    time: Res<Time>,
) {
    for (mut wanderer, wanderer_pos) in query.iter_mut() {
        wanderer.offspring_timer.tick(time.delta());
        if wanderer.offspring_timer.just_finished() {
            let spawn_pos = wanderer_pos.translation + Vec3::new(0.5, 0.0, 0.0);
            let target_pos = get_random_pos_within(WIDTH, HEIGHT);
            let offspring_timer_count = fastrand::u64(10..=20);
            spawn_wanderer(&mut commands, spawn_pos, target_pos, offspring_timer_count);
        }
    }
}

fn update_wanderers(mut query: Query<(&mut Movable, &mut Wanderer, &Transform)>) {
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

fn update_predators(
    mut commands: Commands,
    movable_query: Query<&Transform, (With<Movable>, Without<Predator>)>,
    mut predator_query: Query<(&mut Predator, &mut Movable, &Transform)>,
) {
    let (mut predator, mut movable, predator_pos) = predator_query.single_mut();
    if let Some(movable_entity) = predator.target {
        if let Ok(target_pos) = movable_query.get(movable_entity) {
            let vector_to_target = target_pos.translation - predator_pos.translation;
            if vector_to_target.length() < 25.0 {
                let mut movable_entity = commands.entity(movable_entity);
                debug!("Eaten entity {:?}", movable_entity.id());
                movable_entity.despawn();
                predator.target = None;
            } else {
                movable.direction = vector_to_target.normalize();
            }
        }
    } else {
        movable.direction = Vec3::default();
    }
}

fn choose_target_for_predator(
    wanderer_query: Query<Entity, With<Wanderer>>,
    mut predator_query: Query<&mut Predator>,
) {
    let mut predator = predator_query.single_mut();
    if predator.target.is_none() {
        let wanderers = wanderer_query.iter().collect::<Vec<Entity>>();
        debug!(
            "Choosing next wanderer as target from pool of {}",
            wanderers.len()
        );
        predator.target = select_from_list_at_random(&wanderers);
    }
}

fn select_from_list_at_random(list: &Vec<Entity>) -> Option<Entity> {
    if list.is_empty() {
        None
    } else {
        list.get(fastrand::usize(0..list.len())).cloned()
    }
}

fn get_random_pos_within(width: f32, height: f32) -> Vec3 {
    Vec3::new(
        (fastrand::f32() * (width * 2.0)) - width,
        (fastrand::f32() * (height * 2.0)) - height,
        0.0,
    )
}

fn spawn_wanderer(
    commands: &mut Commands,
    spawn_pos: Vec3,
    target_pos: Vec3,
    offspring_timer_count: u64,
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.50, 0.75),
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
        .insert(Wanderer {
            target_pos,
            offspring_timer: Timer::new(
                Duration::from_secs(offspring_timer_count),
                TimerMode::Once,
            ),
        });
}

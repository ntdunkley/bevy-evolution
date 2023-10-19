use bevy::prelude::*;

mod predator;
mod wanderer;

use predator::Predator;
use wanderer::Wanderer;

const WIDTH: f32 = 600.0;
const HEIGHT: f32 = 300.0;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup).add_systems(
            Update,
            (
                update_wanderers,
                update_predators,
                choose_target_for_predator.before(update_predators),
                move_movables,
                spawn_wanderers,
                boost_predator_speed,
            ),
        );
    }
}

#[derive(Component)]
struct Movable {
    direction: Vec3,
    speed: f32,
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    for _ in 0..20 {
        let spawn_pos = get_random_pos_within(WIDTH, HEIGHT);
        let target_pos = get_random_pos_within(WIDTH, HEIGHT);
        let offspring_timer_count = wanderer::generate_offspring_timer_count();
        wanderer::spawn_wanderer(
            &mut commands,
            spawn_pos,
            target_pos,
            offspring_timer_count,
            wanderer::DEFAULT_COLOUR,
        );
    }
    predator::spawn_predator(&mut commands);
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
            let offspring_timer_count = wanderer::generate_offspring_timer_count();
            let colour = wanderer::get_colour_for_wanderer(wanderer.colour);
            wanderer::spawn_wanderer(
                &mut commands,
                spawn_pos,
                target_pos,
                offspring_timer_count,
                colour,
            );
        }
    }
}

fn update_wanderers(
    mut query: Query<(&mut Movable, &mut Wanderer, &Transform)>,
    predator_query: Query<&Transform, With<Predator>>,
) {
    let predator_pos = predator_query.single();
    for (mut movable, mut wanderer, wanderer_pos) in query.iter_mut() {
        let vector_to_target = wanderer.target_pos - wanderer_pos.translation;
        if vector_to_target.length() < 5.0 {
            wanderer.target_pos = get_random_pos_within(WIDTH, HEIGHT);
        }
        // Move away from predator if too close
        let vector_to_predator = predator_pos.translation - wanderer_pos.translation;
        if vector_to_predator.length() < wanderer::WANDERER_SCARE_DISTANCE {
            movable.direction = -vector_to_predator.normalize();
        } else {
            movable.direction = vector_to_target.normalize();
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
    movable_query: Query<(&Wanderer, &Transform), (With<Movable>, Without<Predator>)>,
    mut predator_query: Query<(&mut Predator, &mut Movable, &Transform)>,
) {
    let (mut predator, mut movable, predator_pos) = predator_query.single_mut();
    if let Some(movable_entity) = predator.target {
        if let Ok((wanderer, target_pos)) = movable_query.get(movable_entity) {
            if wanderer.colour == predator::COLOUR_PREDATOR_IGNORES {
                predator.target = None;
            } else {
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

fn boost_predator_speed(
    mut predator_query: Query<&mut Movable, With<Predator>>,
    keys: Res<Input<KeyCode>>,
) {
    let mut predator = predator_query.single_mut();
    if keys.pressed(KeyCode::Space) {
        predator.speed = predator::PREDATOR_SPEED * predator::PREDATOR_BOOST_MODIFIER;
    } else {
        predator.speed = predator::PREDATOR_SPEED
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

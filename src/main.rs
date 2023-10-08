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
                update_animal_target_pos,
                choose_target_for_fox,
                update_fox_direction,
                move_animals,
                move_fox,
            ),
        )
        .run();
}

#[derive(Component)]
struct Animal {
    target_pos: Vec3,
    direction: Vec3,
    speed: f32,
}

#[derive(Component)]
struct Fox {
    target: Option<Entity>,
    direction: Option<Vec3>,
    speed: f32,
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
            .insert(Animal {
                target_pos,
                direction: (target_pos - spawn_pos).normalize(),
                speed: 100.0,
            });
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
        .insert(Fox {
            target: None,
            direction: None,
            speed: 125.0,
        });
}

fn update_animal_target_pos(mut query: Query<(&mut Animal, &Transform)>) {
    for (mut animal, animal_pos) in query.iter_mut() {
        if animal.target_pos.distance(animal_pos.translation) < 5.0 {
            let target_pos = get_random_pos_within(WIDTH, HEIGHT);
            animal.target_pos = target_pos;
            animal.direction = (target_pos - animal_pos.translation).normalize();
        }
    }
}

fn move_animals(mut query: Query<(&Animal, &mut Transform)>, time: Res<Time>) {
    for (animal, mut animal_pos) in query.iter_mut() {
        animal_pos.translation += animal.direction * animal.speed * time.delta_seconds();
    }
}

fn move_fox(mut query: Query<(&Fox, &mut Transform)>, time: Res<Time>) {
    for (fox, mut fox_pos) in query.iter_mut() {
        if let Some(fox_dir) = fox.direction {
            fox_pos.translation += fox_dir * fox.speed * time.delta_seconds();
        }
    }
}

fn update_fox_direction(
    mut commands: Commands,
    animal_query: Query<&Transform, With<Animal>>,
    mut fox_query: Query<(&mut Fox, &Transform), Without<Animal>>,
) {
    let (mut fox, fox_pos) = fox_query.single_mut();
    if let Some(animal_entity) = fox.target {
        if let Ok(target_pos) = animal_query.get(animal_entity) {
            let vector = target_pos.translation - fox_pos.translation;
            if vector.length() < 25.0 {
                commands.entity(animal_entity).despawn();
                fox.target = None;
                fox.direction = None;
            } else {
                fox.direction = Some(vector.normalize());
            }
        }
    }
}

fn choose_target_for_fox(
    animal_query: Query<Entity, With<Animal>>,
    mut fox_query: Query<&mut Fox, Without<Animal>>,
) {
    let mut fox = fox_query.single_mut();
    if fox.target.is_none() {
        let animal_list = animal_query.iter().collect::<Vec<Entity>>();
        fox.target = get_next_target_for_fox(&animal_list);
    }
}

fn get_next_target_for_fox(animal_list: &Vec<Entity>) -> Option<Entity> {
    if animal_list.len() < 1 {
        None
    } else {
        animal_list
            .get(fastrand::usize(0..animal_list.len()))
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

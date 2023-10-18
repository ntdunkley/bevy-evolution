use crate::simulation::Movable;
use bevy::prelude::{
    default, Color, Commands, Component, Entity, Sprite, SpriteBundle, Vec2, Vec3,
};

pub const PREDATOR_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Predator {
    pub target: Option<Entity>,
}

pub fn spawn_predator(commands: &mut Commands) {
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
            speed: PREDATOR_SPEED,
        });
}

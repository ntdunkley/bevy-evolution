use crate::simulation::Movable;
use bevy::prelude::{
    default, Color, Commands, Component, Sprite, SpriteBundle, Timer, TimerMode, Transform, Vec2,
    Vec3,
};
use std::time::Duration;

pub const WANDERER_SCARE_DISTANCE: f32 = 100.0;
pub const OFFSPRING_TIMER_MIN: u8 = 10;
pub const OFFSPRING_TIMER_MAX: u8 = 15;

#[derive(Component)]
pub struct Wanderer {
    pub target_pos: Vec3,
    pub offspring_timer: Timer,
}

pub fn spawn_wanderer(
    commands: &mut Commands,
    spawn_pos: Vec3,
    target_pos: Vec3,
    offspring_timer_count: u8,
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
                Duration::from_secs(offspring_timer_count as u64),
                TimerMode::Repeating,
            ),
        });
}

pub fn generate_offspring_timer_count() -> u8 {
    fastrand::u8(OFFSPRING_TIMER_MIN..=OFFSPRING_TIMER_MAX)
}

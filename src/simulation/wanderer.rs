use crate::simulation::Movable;
use bevy::prelude::{
    default, Color, Commands, Component, Sprite, SpriteBundle, Timer, TimerMode, Transform, Vec2,
    Vec3,
};
use std::time::Duration;

const WANDERER_SPEED: f32 = 100.0;
pub const WANDERER_SCARE_DISTANCE: f32 = 100.0;
pub const OFFSPRING_TIMER_MIN: u8 = 11;
pub const OFFSPRING_TIMER_MAX: u8 = 14;
pub const DEFAULT_COLOUR: Color = Color::GRAY;

#[derive(Component)]
pub struct Wanderer {
    pub target_pos: Vec3,
    pub offspring_timer: Timer,
    pub colour: Color,
}

pub fn spawn_wanderer(
    commands: &mut Commands,
    spawn_pos: Vec3,
    target_pos: Vec3,
    offspring_timer_count: u8,
    colour: Color,
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: colour,
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
            speed: WANDERER_SPEED,
        })
        .insert(Wanderer {
            target_pos,
            offspring_timer: Timer::new(
                Duration::from_secs(offspring_timer_count as u64),
                TimerMode::Repeating,
            ),
            colour,
        });
}

pub fn generate_offspring_timer_count() -> u8 {
    fastrand::u8(OFFSPRING_TIMER_MIN..=OFFSPRING_TIMER_MAX)
}

pub fn get_colour_for_wanderer(colour: Color) -> Color {
    if fastrand::f32() < 0.05 {
        get_random_colour()
    } else {
        colour
    }
}
pub fn get_random_colour() -> Color {
    let colours = [
        Color::GREEN,
        Color::PINK,
        //Color::PURPLE,
        //Color::SEA_GREEN,
        //Color::TURQUOISE,
        Color::TEAL,
        Color::TOMATO,
        //Color::AQUAMARINE,
        //Color::AZURE,
    ];
    colours[fastrand::usize(0..colours.len())]
}

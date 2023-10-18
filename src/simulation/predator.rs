use bevy::prelude::{Component, Entity};

pub const PREDATOR_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Predator {
    pub target: Option<Entity>,
}

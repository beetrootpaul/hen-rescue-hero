use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component, Clone, Copy)]
pub struct Position(pub Vec2);

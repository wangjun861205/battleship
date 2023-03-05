use bevy::prelude::*;

#[derive(Component)]
pub struct Battleship {
    pub direction: f32,
    pub velocity: f32,
}

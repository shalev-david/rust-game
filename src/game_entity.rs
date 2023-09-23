use bevy::prelude::{Component, Vec2};


#[derive(Component)]
pub struct GameEntity{
    pub width: f32,
    pub height: f32,
    pub entity_position: Vec2,
    pub aim_position: Vec2
}
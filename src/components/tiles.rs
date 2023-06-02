use bevy::prelude::Component;

#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
}

#[derive(Component)]
pub struct DamageArea {
    pub damage: i32,
}

pub enum TileType {
    Clear,
    Blocked,
    Spawner,
    Goal,
}

impl TileType {
    pub fn get_cost(&self) -> u32 {
        match self {
            TileType::Clear => 1,
            TileType::Blocked => 1000,
            TileType::Spawner => 1,
            TileType::Goal => 1,
        }
    }

    pub fn is_valid_spawn(&self) -> bool {
        match self {
            TileType::Spawner | TileType::Goal => false,
            _ => true,
        }
    }
}

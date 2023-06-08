use bevy::prelude::*;

use crate::{
    components::tiles::{Tile, TilePath, TileType},
    resources::GameAssets,
};

pub fn on_tile_type_changed(
    game_assets: ResMut<GameAssets>,
    mut entities: Query<(&mut Handle<ColorMaterial>, &Tile, Entity), Changed<Tile>>,
    tiles_path: Query<&TilePath>,
) {
    for (mut material, tile, entity) in entities.iter_mut() {
        let mat = if tiles_path.contains(entity) {
            game_assets.path_tile_material.clone()
        } else {
            match tile.tile_type {
                TileType::Clear => game_assets.clear_tile_material.clone(),
                TileType::Blocked => game_assets.blocked_tile_material.clone(),
                TileType::Spawner => game_assets.spawner_tile_material.clone(),
                TileType::Goal => game_assets.goal_tile_material.clone(),
            }
        };
        *material = mat;
    }
}

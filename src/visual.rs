use bevy::prelude::*;

use crate::{
    components::tiles::{Tile, TileType},
    resources::GameAssets,
};

pub fn on_tile_type_changed(
    game_assets: ResMut<GameAssets>,
    mut entities: Query<(&mut Handle<ColorMaterial>, &Tile), Or<(Changed<Tile>, Added<Tile>)>>,
) {
    for (mut material, tile) in entities.iter_mut() {
        let mat = match tile.tile_type {
            TileType::Clear => game_assets.clear_tile_material.clone(),
            TileType::Blocked => game_assets.blocked_tile_material.clone(),
            TileType::Spawner => game_assets.spawner_tile_material.clone(),
            TileType::Goal => game_assets.goal_tile_material.clone(),
        };
        *material = mat;
    }
}

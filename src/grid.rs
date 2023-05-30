use std::collections::HashMap;

use crate::{
    components::{hexgrid::HexGrid, tiles::*},
    resources::GameAssets,
};
use bevy::prelude::*;
use hexx::{Hex, HexLayout};
use rand::Rng;

use crate::resources::MapConfig;

pub fn setup_grid(
    map_config: ResMut<MapConfig>,
    mut commands: Commands,
    game_assets: ResMut<GameAssets>,
) {
    let layout = HexLayout {
        hex_size: map_config.hex_size,
        ..default()
    };

    let mut rng = rand::thread_rng();

    let board_entity = commands
        .spawn(SpatialBundle {
            visibility: Visibility::Visible,
            ..Default::default()
        })
        .id();

    let tiles_entities: HashMap<_, _> = Hex::ZERO
        .spiral_range(0..=map_config.map_radius)
        .enumerate()
        .map(|(_i, coord)| {
            let tile_type = get_random_tile_type(rng.gen_range(0..=1));
            let pos = layout.hex_to_world_pos(coord);
            let material = match tile_type {
                TileType::Clear => game_assets.clear_tile_material.clone(),
                TileType::Blocked => game_assets.blocked_tile_material.clone(),
                _ => unreachable!(),
            };
            let entity: Entity = commands
                .spawn(ColorMesh2dBundle {
                    mesh: game_assets.bestagone_mesh.clone().into(),
                    material,
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0).with_scale(Vec3::splat(1.)),
                    ..default()
                })
                .insert(Tile { tile_type })
                .set_parent(board_entity)
                .id();
            (coord, entity)
        })
        .collect();
    commands.entity(board_entity).insert(HexGrid {
        tiles_entities,
        layout,
        selected_hex: Hex::ZERO,
        tower_hexs: HashMap::new(),
    });
}

fn get_random_tile_type(random: u8) -> TileType {
    match random {
        0 => TileType::Clear,
        1 => TileType::Blocked,
        _ => unreachable!(),
    }
}

pub fn setup_spawners() {
    //
}

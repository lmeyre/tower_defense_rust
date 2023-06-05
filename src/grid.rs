use std::collections::HashMap;

use crate::{
    components::{
        enemies::{Health, Spawner},
        hexgrid::HexGrid,
        tiles::*,
    },
    resources::GameAssets,
};
use bevy::prelude::*;
use hexx::{DiagonalDirection, Hex, HexLayout};
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

    let mut tiles_entities: HashMap<_, _> = Hex::ZERO
        .spiral_range(1..=map_config.map_radius)
        .enumerate()
        .map(|(_i, coord)| {
            let tile_type = get_random_tile_type(rng.gen_range(0..=1));
            let pos = layout.hex_to_world_pos(coord);
            // let material = match tile_type {
            //     TileType::Clear => game_assets.clear_tile_material.clone(),
            //     TileType::Blocked => game_assets.blocked_tile_material.clone(),
            //     _ => unreachable!(),
            // };
            let entity: Entity = commands
                .spawn(ColorMesh2dBundle {
                    mesh: game_assets.bestagone_mesh.clone().into(),
                    //material,
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0).with_scale(Vec3::splat(1.)),
                    ..default()
                })
                .insert(Tile { tile_type })
                .set_parent(board_entity)
                .id();
            (coord, entity)
        })
        .collect();

    let origin: Entity = commands
        .spawn(Tile {
            tile_type: TileType::Goal,
        })
        .set_parent(board_entity)
        .id();
    tiles_entities.insert(Hex::ZERO, origin);

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

pub fn setup_spawners(
    mut commands: Commands,
    map_config: ResMut<MapConfig>,
    grid: Query<&HexGrid>,
) {
    let mut rng = rand::thread_rng();
    if let Ok(grid) = grid.get_single() {
        for direction in DiagonalDirection::ALL_DIRECTIONS {
            let hex_iterator = Hex::ZERO.ring_edge(map_config.map_radius, direction);
            let index = rng.gen_range(0..hex_iterator.len());
            if let Some(spawner) = hex_iterator.skip(index).next() {
                if let Some(entity) = grid.tiles_entities.get(&spawner) {
                    commands.entity(*entity).insert(Spawner { hex: spawner });
                }
            }
        }
    }
}

// Deal damage to entities
pub fn damage_entities(
    mut entities: Query<(&mut Health, &Transform)>,
    damaging_tiles: Query<(&DamageArea, &Tile, Entity)>,
    grid: Query<&HexGrid>,
) {
    for (mut health, position) in entities.iter_mut() {
        if let Ok(grid) = grid.get_single() {
            // Getting the hex entity at the position of the enemy
            let hex = grid.layout.world_pos_to_hex(Vec2 {
                x: position.translation.x,
                y: position.translation.y,
            });
            if let Some(tile_entity) = grid.tiles_entities.get(&hex) {
                // If it carry damage, apply it
                if let Ok((damaging_tile, _, _)) = damaging_tiles.get(*tile_entity) {
                    health.health = health.health.saturating_sub(damaging_tile.damage);
                }
            }
        }
    }
}

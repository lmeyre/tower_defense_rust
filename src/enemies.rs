use bevy::prelude::*;
use hexx::{algorithms::a_star, Hex};

use crate::{
    components::{
        enemies::{EnemiesSpawnTimer, Enemy, EnemyBundle, Health, Movement, Spawner, TilePath},
        hexgrid::HexGrid,
        tiles::Tile,
    },
    resources::{GameAssets, GameConfig},
};

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Query<&mut EnemiesSpawnTimer>,
    game_config: Res<GameConfig>,
    game_assets: Res<GameAssets>,
    spawners: Query<&Spawner>,
    grid: Query<&HexGrid>,
) {
    if let Ok(mut timer) = timer.get_single_mut() {
        if timer.tick(time.delta()).finished() {
            timer.reset();
            if let Ok(grid) = grid.get_single() {
                for spawner in spawners.iter() {
                    let health: Health = Health {
                        health: Health::get_random_health(
                            game_config.enemies_min_health,
                            game_config.enemies_max_health,
                        ),
                    };
                    let size = health.get_size(game_config.as_ref());
                    let position = grid.layout.hex_to_world_pos(spawner.hex);
                    commands
                        .spawn(ColorMesh2dBundle {
                            mesh: game_assets.circle_mesh.clone().into(),
                            material: game_assets.enemy_material.clone(),
                            transform: Transform {
                                translation: (Vec3 {
                                    x: (position.x),
                                    y: (position.y),
                                    z: (0.3),
                                }),
                                scale: Vec3 {
                                    x: size,
                                    y: size,
                                    z: size,
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .insert(EnemyBundle {
                            movement: Movement {
                                speed: Movement::get_random_speed(
                                    game_config.enemies_min_speed,
                                    game_config.enemies_max_speed,
                                ),
                                spawner_hex: spawner.hex,
                                current_target: *spawner.path.get(0).unwrap(),
                            },
                            health,
                            enemy: Enemy {},
                        });
                }
            }
        }
    }
}

pub fn on_damage_taken(
    mut commands: Commands,
    mut damaged: Query<(&Health, Entity, &mut Transform), Changed<Health>>,
    game_config: Res<GameConfig>,
) {
    for (health, entity, mut transform) in damaged.iter_mut() {
        let size = health.get_size(game_config.as_ref());
        transform.scale = Vec3 {
            x: size,
            y: size,
            z: size,
        };
        if health.health == 0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn move_enemies(
    mut commands: Commands,
    mut enemies: Query<(&mut Movement, &mut Transform, Entity)>,
    grid: Query<&HexGrid>,
    spawners: Query<&Spawner>,
    time: Res<Time>,
) {
    //Enemies will look where they are in the path of their spawner
    //They will have a target hex, once distance is small, they go to the next
    if let Ok(grid) = grid.get_single() {
        for (mut movement, mut transform, entity) in enemies.iter_mut() {
            let target_pos = grid
                .layout
                .hex_to_world_pos(movement.current_target)
                .extend(0.);
            let distance = transform.translation.distance(target_pos);
            // Change target when tile is reached
            if distance < 1. {
                // If goal is reached
                if movement.current_target == Hex::ZERO {
                    commands.entity(entity).despawn();
                } else if let Some(spawner_entity) =
                    grid.spawner_entities.get(&movement.spawner_hex)
                {
                    if let Ok(spawner) = spawners.get(*spawner_entity) {
                        if let Some(index) = spawner
                            .path
                            .iter()
                            .position(|x| *x == movement.current_target)
                        {
                            if let Some(next_hex) = spawner.path.get(index + 1) {
                                movement.current_target = *next_hex;
                            }
                        }
                    }
                }
            }
            //Move the entites
            let translation = &mut transform.translation;
            *translation = translation.lerp(target_pos, movement.speed * time.delta_seconds());
        }
    }
}

pub fn refresh_spawners_path(
    mut commands: Commands,
    tiles: Query<&Tile, Changed<Tile>>,
    mut spawners: Query<&mut Spawner>,
    grid: Query<&HexGrid>,
    tiles_path: Query<(&TilePath, Entity)>,
) {
    if let Ok(grid) = grid.get_single() {
        for (_, entity) in tiles_path.iter() {
            commands.entity(entity).remove::<TilePath>();
        }
        for mut spawner in spawners.iter_mut() {
            if let Some(path) = a_star(spawner.hex, Hex::ZERO, |hex| {
                if let Some(entity) = grid.tiles_entities.get(&hex) {
                    if let Ok(tile) = tiles.get(*entity) {
                        Some(tile.tile_type.get_cost())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }) {
                for spawner_path_tile in path.clone() {
                    if let Some(entity) = grid.tiles_entities.get(&spawner_path_tile) {
                        commands.entity(*entity).insert(TilePath {});
                    }
                }
                spawner.path = path;
            }
        }
    }
}

pub fn on_spawner_created(
    tiles: Query<&Tile>,
    mut spawners: Query<&mut Spawner, Added<Spawner>>,
    grid: Query<&HexGrid>,
) {
    if let Ok(grid) = grid.get_single() {
        for mut spawner in spawners.iter_mut() {
            if let Some(path) = a_star(spawner.hex, Hex::ZERO, |hex| {
                if let Some(entity) = grid.tiles_entities.get(&hex) {
                    if let Ok(tile) = tiles.get(*entity) {
                        Some(tile.tile_type.get_cost())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }) {
                spawner.path = path;
            }
        }
    }
}

// fn process_spawners_path(
//     tiles: &Query<&Tile>,
//     spawners: &mut Query<&mut Spawner>,
//     grid: &Query<&HexGrid>,
// ) {
//     if let Ok(grid) = grid.get_single() {
//         for mut spawner in spawners.iter_mut() {
//             if let Some(path) = a_star(spawner.hex, Hex::ZERO, |hex| {
//                 if let Some(entity) = grid.tiles_entities.get(&hex) {
//                     if let Ok(tile) = tiles.get(*entity) {
//                         Some(tile.tile_type.get_cost())
//                     } else {
//                         None
//                     }
//                 } else {
//                     None
//                 }
//             }) {
//                 spawner.path = path;
//             }
//         }
//     }
// }

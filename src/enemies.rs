use bevy::prelude::*;
use hexx::{algorithms::a_star, Hex};

use crate::{
    components::{
        enemies::{EnemiesSpawnTimer, Enemy, EnemyBundle, Health, Movement, Spawner},
        hexgrid::HexGrid,
        tiles::Tile,
    },
    resources::{GameAssets, GameConfig},
};

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Query<&mut EnemiesSpawnTimer>,
    gameconfig: Res<GameConfig>,
    game_assets: Res<GameAssets>,
    spawners: Query<&Spawner>,
) {
    if let Ok(mut timer) = timer.get_single_mut() {
        if timer.tick(time.delta()).finished() {
            return;
        } else {
            timer.reset();
        }
        for spawner in spawners.iter() {
            commands
                .spawn(ColorMesh2dBundle {
                    mesh: game_assets.circle_mesh.clone().into(),
                    material: game_assets.enemy_material.clone(),
                    transform: Transform {
                        translation: (Vec3 {
                            x: (spawner.hex.x as f32),
                            y: (spawner.hex.y as f32),
                            z: (0.),
                        }),
                        ..default()
                    },
                    ..default()
                })
                .insert(EnemyBundle {
                    movement: Movement {
                        speed: Movement::get_random_speed(
                            gameconfig.enemies_min_speed,
                            gameconfig.enemies_max_speed,
                        ),
                        spawner_hex: spawner.hex,
                        current_target: *spawner.path.get(0).unwrap(),
                    },
                    health: Health {
                        health: Health::get_random_health(
                            gameconfig.enemies_min_health,
                            gameconfig.enemies_max_health,
                        ),
                    },
                    enemy: Enemy {},
                });
        }
    }
}

pub fn on_damage_taken(mut commands: Commands, damaged: Query<(&Health, Entity), Changed<Health>>) {
    for (health, entity) in damaged.iter() {
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
    tiles: Query<&Tile, Changed<Tile>>,
    mut spawners: Query<&mut Spawner>,
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

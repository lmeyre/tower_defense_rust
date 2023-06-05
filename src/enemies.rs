use bevy::prelude::*;

use crate::{
    components::enemies::{EnemiesSpawnTimer, Enemy, EnemyBundle, Health, Movement, Spawner},
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

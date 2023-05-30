use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    components::{
        enemies::{EnemiesSpawnTimer, EnemyBundle, Health, Movement, Spawner},
        tiles,
    },
    resources::{GameAssets, GameConfig},
};

pub fn spawn_enemies(
    commands: &mut Commands,
    materials: Res<GameAssets>,
    time: Res<Time>,
    mut timer: Query<&EnemiesSpawnTimer>,
    gameconfig: Res<GameConfig>,
    game_assets: Res<GameAssets>,
    spawners: Query<&Spawner>,
) {
    if let Ok(timer) = timer.get_single_mut() {
        if timer.0.tick(time.delta()).finished() {
            return;
        } else {
            timer.0.reset();
        }
        for spawner in spawners.iter() {
            commands
                .spawn(ColorMesh2dBundle {
                    mesh: game_assets.circle_mesh.clone().into(),
                    material: game_assets.enemy_material,
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
                })
                .id();
        }
    }
}

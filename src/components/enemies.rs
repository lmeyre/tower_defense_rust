use bevy::{
    prelude::{Bundle, Component},
    time::Timer,
};
use hexx::Hex;
use rand::Rng;

#[derive(Bundle)]
pub struct EnemyBundle {
    movement: Movement,
    health: Health,
}

#[derive(Component)]
pub struct Health {
    pub health: i32,
}

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
}

impl Health {
    pub fn get_random_health(min: i32, max: i32) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }

    pub fn get_size(&self) -> u32 {
        0
    }
}

impl Movement {
    pub fn get_random_speed(min: f32, max: f32) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }
}

#[derive(Component)]
pub struct Spawner {
    pub hex: Hex,
}

#[derive(Debug, Component)]
pub struct EnemiesSpawnTimer(pub Timer);

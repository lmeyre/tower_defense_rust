use bevy::{
    prelude::{Bundle, Component, Deref, DerefMut},
    time::Timer,
};
use hexx::Hex;
use rand::Rng;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub movement: Movement,
    pub health: Health,
    pub enemy: Enemy,
}

#[derive(Component)]
pub struct Health {
    pub health: u32,
}

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
    pub spawner_hex: Hex,
    pub current_target: Hex,
}

#[derive(Component)]
pub struct Enemy {}

impl Health {
    pub fn get_random_health(min: u32, max: u32) -> u32 {
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
    pub path: Vec<Hex>,
}

#[derive(Debug, Component, Deref, DerefMut)]
pub struct EnemiesSpawnTimer(pub Timer);

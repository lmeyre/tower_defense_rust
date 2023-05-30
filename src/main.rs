mod camera;
mod grid;
mod input;
mod resources;
mod ui;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
pub mod components;
mod enemies;
mod initialize;
mod towers;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        // Resources
        .init_resource::<resources::MapConfig>()
        .init_resource::<resources::GameConfig>()
        // Startup Systems
        .add_startup_system(initialize::initialize)
        .add_startup_system(camera::setup_camera)
        .add_startup_system(grid::setup_grid)
        .add_startup_system(grid::setup_spawners)
        // Runtime Systems
        .add_system(input::handle_input)
        .add_system(towers::spawn_tower)
        .add_system(enemies::spawn_enemies)
        .add_system(ui::display_ui)
        .run();
}

mod camera;
mod grid;
mod input;
mod resources;
mod ui;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use input::{LeftClickEvent, RightClickEvent};
pub mod components;
mod enemies;
mod initialize;
mod terrain;
mod towers;
mod visual;

//TODO
//Communication Channels
//Recreate board

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_500.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        //Events
        .add_event::<LeftClickEvent>()
        .add_event::<RightClickEvent>()
        // Resources
        .init_resource::<resources::MapConfig>()
        .init_resource::<resources::GameConfig>()
        // Startup Systems
        .add_startup_systems(
            (
                initialize::initialize,
                apply_system_buffers,
                grid::setup_grid,
            )
                .chain(),
        )
        .add_startup_system(camera::setup_camera)
        .add_startup_system(grid::setup_spawners)
        // Runtime Systems
        .add_system(input::handle_input)
        .add_system(towers::spawn_tower)
        .add_system(enemies::spawn_enemies)
        .add_system(grid::damage_entities)
        .add_system(ui::display_ui)
        .add_system(visual::on_tile_type_changed)
        .add_system(towers::on_tower_spawned)
        .add_system(enemies::refresh_spawners_path)
        .add_system(terrain::change_terrain)
        .run();
}

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

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    Starting,
    Running,
}

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
                // apply_system_buffers,
                // grid::setup_grid,
                // apply_system_buffers,
                // grid::setup_spawners,
                // apply_system_buffers,
                // enemies::on_spawner_created,
            )
                .chain(),
        )
        .add_startup_system(camera::setup_camera)
        // State
        .add_state::<AppState>()
        // Starting systems
        .add_systems(
            (
                grid::setup_grid,
                apply_system_buffers,
                grid::setup_spawners,
                apply_system_buffers,
                enemies::on_spawner_created,
            )
                .chain()
                .in_schedule(OnEnter(AppState::Starting)),
        )
        // Runnings Systems
        .add_systems(
            (
                input::handle_input,
                towers::spawn_tower,
                enemies::spawn_enemies,
                towers::damage_entities,
                ui::display_ui,
                visual::on_tile_type_changed,
                towers::on_tower_spawned,
                enemies::refresh_spawners_path,
                terrain::change_terrain,
                enemies::move_enemies,
                enemies::on_damage_taken,
            )
                .in_set(OnUpdate(AppState::Running)),
        )
        .run();
}

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    components::{hexgrid::HexGrid, tiles::Tile, towers::Tower},
    input::RightClickEvent,
    resources::GameAssets,
};

pub fn spawn_tower(
    mut commands: Commands,
    mut right_click_event: EventReader<RightClickEvent>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut board: Query<(&mut HexGrid, Entity)>,
    tiles: Query<&Tile>,
    game_assets: Res<GameAssets>,
) {
    //Check if valid -> not on another tower / not goal / not spawn
    if right_click_event.iter().last().is_some() {
        if let Ok(windows) = windows.get_single() {
            if let Some(pos) = windows.cursor_position() {
                if let Ok((mut grid, board_entity)) = board.get_single_mut() {
                    let pos = pos - Vec2::new(windows.width(), windows.height()) / 2.0;

                    let hex_pos = grid.layout.world_pos_to_hex(pos);

                    let tile_entity = grid.tiles_entities.get(&hex_pos);
                    if let Some(tile_entity) = tile_entity {
                        let tile = tiles.get(*tile_entity);
                        if let Ok(t) = tile {
                            if t.tile_type.is_valid_spawn()
                                && !grid.tower_hexs.contains_key(&hex_pos)
                            {
                                let tower_entity = commands
                                    .spawn(ColorMesh2dBundle {
                                        mesh: game_assets.square_mesh.clone().into(),
                                        material: game_assets.tower_material.clone(),
                                        transform: Transform {
                                            translation: (Vec3 {
                                                x: (hex_pos.x as f32),
                                                y: (hex_pos.y as f32),
                                                z: (0.),
                                            }),
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .insert(Tower { hex: hex_pos })
                                    .set_parent(board_entity)
                                    .id();
                                grid.tower_hexs.insert(hex_pos, tower_entity);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn on_tower_spawned(new_towers: Query<&Tower>) {

    //Somehow get tiles around  -> spiral range
    //Put Damage on each tiles
    // Should put damage tile on tiles around here, or its on this feature i must do a reaction from other system that listen to this ?
}

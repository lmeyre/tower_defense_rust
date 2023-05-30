use bevy::prelude::*;

pub fn handle_input(
    mouse_button_input: ResMut<Input<MouseButton>>,
    mut left_click_event: EventWriter<LeftClickEvent>,
    mut right_click_event: EventWriter<RightClickEvent>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        left_click_event.send(LeftClickEvent);
    }

    if mouse_button_input.just_pressed(MouseButton::Right) {
        right_click_event.send(RightClickEvent);
    }
}

pub struct LeftClickEvent;
pub struct RightClickEvent;

// /// Input interaction
// pub fn handle_input(
//     map_config: Res<MapConfig>,
//     windows: Query<&Window, With<PrimaryWindow>>,
//     tiles: Query<(Entity, &mut Transform, &Tile)>,
//     mut current: Local<Hex>,
//     mut hexgrid: Query<&mut HexGrid>,
// ) {
//     let mut hexgrid = hexgrid.single_mut();
//     let window = windows.single();
//     if let Some(pos) = window.cursor_position() {
//         let pos = pos - Vec2::new(window.width(), window.height()) / 2.0;
//         let hex_pos = hexgrid.layout.world_pos_to_hex(pos);

//         if hex_pos == *current {
//             return;
//         }
//         *current = hex_pos;

//         // display_tiles_range(hex_pos, &map_config, tiles, &mut hexgrid);
//     }
// }

// fn display_tiles_range(
//     hex_pos: Hex,
//     map_config: &MapConfig, // Is this right ? Or should it just be without query? (Cant make it work without)
//     mut tiles: Query<(Entity, &mut Transform, &Tile)>,
//     hexgrid: &mut HexGrid,
// ) {
//     let field_of_movement = field_of_movement(hex_pos, map_config.budget, |hex| {
//         hexgrid.tiles_entities.get(&hex).and_then(|entity| {
//             //Some(
//             tiles
//                 .get_component::<Tile>(*entity)
//                 .map(|tile| tile.tile_type.get_cost())
//                 .ok()
//             // .unwrap()
//             // .tile_type
//             // .get_cost(),
//             // )
//         })
//     });

//     let reachable_entities: HashSet<_> = field_of_movement
//         .into_iter()
//         .filter_map(|h| hexgrid.tiles_entities.get(&h).map(|&ent| ent))
//         .collect();
//     for (entity, mut transform, _) in tiles.iter_mut() {
//         if reachable_entities.contains(&entity) {
//             *transform = transform.with_scale(Vec3::splat(0.9));
//         } else {
//             *transform = transform.with_scale(Vec3::splat(1.));
//         }
//     }

//     hexgrid.reachable_tiles_entities = reachable_entities;
// }

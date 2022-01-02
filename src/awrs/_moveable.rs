// use bevy::prelude::*;

// use super::{
//     _cell::Cell, _cell::CellChange, constants::TILE_SIZE, map::GameMap,
//     register_inputs::InputEvent, unit::AddUnitMoveStepEvent,
// };
// pub struct Moveable;

// pub struct MoveEvent {
//     pub cell_change: CellChange,
// }

// pub fn move_freely(
//     mut q_moveable: Query<(&mut Transform, &mut Cell), With<Moveable>>,
//     q_game_map: Query<&GameMap>,
//     mut ev_input: EventReader<InputEvent>,
//     mut ev_move_step: EventWriter<AddUnitMoveStepEvent>,
// ) {
//     let game_map = q_game_map
//         .single()
//         .expect("Trying to move a unit when there is no map?!");

//     for InputEvent(cell_change) in ev_input.iter() {
//         info!("Move Event receieved!");
//         for (mut transform, mut cell) in q_moveable.iter_mut() {
//             // Too many casts
//             let translation_change =
//                 Vec3::new(cell_change.x as f32, cell_change.y as f32, 0.0) * TILE_SIZE;
//             let new_x = cell.x as isize + cell_change.x;
//             let new_y = cell.y as isize + cell_change.y;

//             if new_x >= 0 && new_x <= game_map.width as isize {
//                 cell.x = new_x as usize;
//                 transform.translation.x += translation_change.x;
//             }
//             if new_y >= 0 && new_y <= game_map.height as isize {
//                 cell.y = new_y as usize;
//                 ev_move_step.send(AddUnitMoveStepEvent(Cell {
//                     x: new_x as usize,
//                     y: new_y as usize,
//                 }));
//                 transform.translation.y += translation_change.y;
//             }
//         }
//     }
// }

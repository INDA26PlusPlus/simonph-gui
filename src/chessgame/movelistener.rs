use bevy::prelude::*;
use crate::chessgame::event::{BoardUpdated, SquareDeselect, SquareSelect};

use super::movevalidator::MetaBoard;
use super::states::MoveState;

#[derive(Event)]
pub struct SquareClicked{
    pub square:(usize,usize),
}
#[derive(Resource)]
pub struct MoveListener{
    pub start:Option<(usize,usize)>,
    pub end:Option<(usize,usize)>
}
pub fn update_start(square:On<SquareSelect>, mut move_listener: ResMut<MoveListener>){
    move_listener.start = Some(square.square);
}
pub fn reset_move_listener(_:On<SquareDeselect>, mut move_listener: ResMut<MoveListener>){
    move_listener.start = None;
    move_listener.end = None;
}
impl Default for MoveListener{
    fn default() -> Self {
        MoveListener{start:None, end:None}
    }
}
pub fn none_click_listener(
    click : On<SquareClicked>,
    meta_board: Res<MetaBoard>,
    mut commands: Commands,
){
    if meta_board.is_same_colour_piece(click.square){
        commands.trigger(SquareSelect{square:click.square});
    }
}
pub fn has_active_listener(
    click : On<SquareClicked>,
    mut meta_board: ResMut<MetaBoard>,
    mut commands: Commands,
    state: Res<MoveListener>
){
    let square = click.square;
    let start_pos = match state.start{
        Some(v) => v,
        _ => panic!("no start pos in active state"),
    };
    if let Ok(..) = meta_board.make_move(start_pos,square,'q'){
        commands.trigger(SquareDeselect{});
        commands.trigger(BoardUpdated{});
        return;
    }
    if meta_board.is_same_colour_piece(square){
        if let Some(state_square) = state.start{
            if state_square == square{
                commands.trigger(SquareDeselect);
                return;
            }
        }
        commands.trigger(SquareSelect{square});
        return;
    }
    commands.trigger(SquareDeselect{});
}
pub fn has_active_piece_listener(
    click : On<SquareClicked>,
    mut meta_board: ResMut<MetaBoard>,
    mut commands: Commands,
    state: Res<MoveListener>,
    mut next_state: ResMut<NextState<MoveState>>
){
    let square = click.square;
    let start_pos = match state.start{
        Some(v) => v,
        _ => panic!("no start pos in active state"),
    };
    if let Ok(..) = meta_board.make_move(start_pos,square,'q'){
        commands.trigger(SquareDeselect{});
        commands.trigger(BoardUpdated{});
        return;
    }
    if meta_board.is_same_colour_piece(square){
        if let Some(state_square) = state.start{
            if state_square == square{
                next_state.set(MoveState::ActiveSquare);
                return;
            }
        }
        commands.trigger(SquareSelect{square});
        return;
    }
    commands.trigger(SquareDeselect{});
}
//pub fn promotion(){}
// pub fn click_listener(
//     click : On<SquareClicked>,
//     state: Res<State<MoveState>>,
//     mut highlight_state: ResMut<NextState<HighlightStage>>,
//     mut next_state: ResMut<NextState<MoveState>>,
//     mut meta_board: ResMut<MetaBoard>,
//     mut commands:Commands
// ){
//     let square = click.square;
//     highlight_state.set(HighlightStage::Clear);
//     match state.get(){
//         MoveState::ActiveSquare(x,y) => {
//             if meta_board.pick_promotion((*x,*y), square){
//                 next_state.set(MoveState::Promotion((*x,*y), square));
//                 return;
//             }
//             match meta_board.make_move((*x,*y), square, 'q'){
//                 Ok(_) => {
//                     next_state.set(MoveState::None);
//                     commands.trigger(BoardUpdated{});
//                     println!("make move {} {} to {} {}", x,y, square.0,square.1);
//                 },
//                 Err(_) =>{
//                     if meta_board.is_same_colour_piece(square) && (square.0 != *x || square.1 != *y){
//                         next_state.set(MoveState::ActiveSquare(square.0, square.1));
//                     }
//                     else{
//                         next_state.set(MoveState::None);
//                     }

//                 } 
//             }
//         }
//         MoveState::None => {
//             //commands.entity(click.entity).insert(super::board::SquareHighlight{});
//             if meta_board.is_same_colour_piece(square){
//                 next_state.set(MoveState::ActiveSquare(square.0, square.1));
//             }
            
//         }
//         _ => {},
//     }

// }
use bevy::prelude::*;
use super::event::BoardUpdated;
use super::movevalidator::MetaBoard;
use super::states::BoardState;
pub fn check_checkmate(_:On<BoardUpdated>, meta_board:Res<MetaBoard>, mut next_state:ResMut<NextState<BoardState>>){
    if meta_board.is_checkmate(){
        next_state.set(BoardState::GameOver);
    }
}
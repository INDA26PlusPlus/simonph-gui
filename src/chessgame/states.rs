use bevy::prelude::*;

use crate::chessgame::event::{SquareDeselect, SquareSelect};
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum HighlightStage{
    #[default]
    None,
    Clear,
    Active,
}

pub fn set_clear_observer(_:On<SquareSelect>, mut next_state: ResMut<NextState<HighlightStage>>){
    next_state.set(HighlightStage::Clear);
}

pub fn set_clear(mut next_state: ResMut<NextState<HighlightStage>>){
    next_state.set(HighlightStage::Clear);
}

pub fn redirect_clear(mut next_state: ResMut<NextState<HighlightStage>>, state:Res<State<MoveState>>){
    match state.get(){
        &MoveState::None => next_state.set(HighlightStage::None),
        &MoveState::ActiveSquare => next_state.set(HighlightStage::Active),
        //_ => next_state.set(HighlightStage::None),
    }
}
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MoveState{
    ActiveSquare,
    //ActivePiece(i8,i8),
    //Promotion,
    #[default]
    None
}

pub fn set_none(_:On<SquareDeselect>,mut next_state:ResMut<NextState<MoveState>>){
    next_state.set(MoveState::None);
}
pub fn set_active_square(_:On<SquareSelect>, mut next_state:ResMut<NextState<MoveState>>){
    next_state.set(MoveState::ActiveSquare);
}
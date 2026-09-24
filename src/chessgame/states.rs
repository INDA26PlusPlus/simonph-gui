use bevy::prelude::*;

use crate::chessgame::event::*;
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
        &MoveState::ActivePiece => next_state.set(HighlightStage::Active),
        _ => next_state.set(HighlightStage::None),
    }
}
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MoveState{
    #[default]
    Off,
    ActiveSquare,
    ActivePiece,
    Promotion,
    None
}
pub fn activate_movestate(mut next_state:ResMut<NextState<MoveState>>){
    next_state.set(MoveState::None);
}
pub fn set_none(_:On<SquareDeselect>,mut next_state:ResMut<NextState<MoveState>>){
    next_state.set(MoveState::None);
}
pub fn set_active_square(_:On<SquareSelect>, mut next_state:ResMut<NextState<MoveState>>){
    next_state.set(MoveState::ActivePiece);
}
pub fn set_promotion(_:On<CallForPromotion>, mut next_state:ResMut<NextState<MoveState>>){
    next_state.set(MoveState::Promotion);
}
pub fn turn_off(mut next_state: ResMut<NextState<MoveState>>){
    next_state.set(MoveState::Off);
}
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BoardState{
    #[default]
    Hidden,
    Startup,
    Playing,
    GameOver,
}
pub fn set_playing(mut next_state: ResMut<NextState<BoardState>>){
    next_state.set(BoardState::Playing);
}
pub fn set_startup(mut next_state: ResMut<NextState<BoardState>>){
    next_state.set(BoardState::Startup);
}
pub fn set_hidden(mut next_state: ResMut<NextState<BoardState>>){
    next_state.set(BoardState::Hidden);
}
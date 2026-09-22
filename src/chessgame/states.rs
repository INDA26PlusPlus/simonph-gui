use bevy::prelude::*;
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum HighlightStage{
    #[default]
    None,
    Clear,
    Active,
}
pub fn set_active(mut next_state: ResMut<NextState<HighlightStage>>){
    next_state.set(HighlightStage::Active);
}
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MoveState{
    ActiveSquare(usize,usize),
    //ActivePiece(i8,i8),
    //Promotion(i8,i8,i8,i8),
    #[default]
    None
}
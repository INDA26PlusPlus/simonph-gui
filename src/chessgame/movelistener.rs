use bevy::prelude::*;
use crate::chessgame::event::BoardUpdated;

use super::movevalidator::MetaBoard;
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MoveState{
    ActiveSquare(usize,usize),
    //ActivePiece(i8,i8),
    //Promotion(i8,i8,i8,i8),
    #[default]
    None
}

#[derive(Event)]
pub struct SquareClicked{
    pub square:(usize,usize),
}

pub fn click_listener(
    click : On<SquareClicked>,
    state: Res<State<MoveState>>,
    mut next_state: ResMut<NextState<MoveState>>,
    mut meta_board: ResMut<MetaBoard>,
    mut commands:Commands

    
){
    let square = click.square;
    match state.get(){
        MoveState::ActiveSquare(x,y) => {
            match meta_board.make_move((*x,*y), square, 'q'){
                Ok(_) => {
                    next_state.set(MoveState::None);
                    commands.trigger(BoardUpdated{});
                },
                Err(_) => next_state.set(MoveState::None)
            }
            println!("make move {} {} to {} {}", x,y, square.0,square.1);
            next_state.set(MoveState::None);
        }
        MoveState::None => {
            //commands.entity(click.entity).insert(super::board::SquareHighlight{});
            next_state.set(MoveState::ActiveSquare(square.0, square.1));
        }

    }

}
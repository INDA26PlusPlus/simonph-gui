use bevy::prelude::*;
use crate::chessgame::event::{BoardUpdated};
use super::states::*;

use super::movevalidator::MetaBoard;


#[derive(Event)]
pub struct SquareClicked{
    pub square:(usize,usize),
}

pub fn click_listener(
    click : On<SquareClicked>,
    state: Res<State<MoveState>>,
    mut highlight_state: ResMut<NextState<HighlightStage>>,
    mut next_state: ResMut<NextState<MoveState>>,
    mut meta_board: ResMut<MetaBoard>,
    mut commands:Commands
){
    let square = click.square;
    highlight_state.set(HighlightStage::Clear);
    match state.get(){
        MoveState::ActiveSquare(x,y) => {
            match meta_board.make_move((*x,*y), square, 'q'){
                Ok(_) => {
                    next_state.set(MoveState::None);
                    commands.trigger(BoardUpdated{});
                    println!("make move {} {} to {} {}", x,y, square.0,square.1);
                },
                Err(_) =>{
                    if meta_board.is_same_colour_piece(square) && (square.0 != *x || square.1 != *y){
                        next_state.set(MoveState::ActiveSquare(square.0, square.1));
                    }
                    else{
                        next_state.set(MoveState::None);
                    }

                } 
            }
        }
        MoveState::None => {
            //commands.entity(click.entity).insert(super::board::SquareHighlight{});
            if meta_board.is_same_colour_piece(square){
                next_state.set(MoveState::ActiveSquare(square.0, square.1));
            }
            
        }

    }

}
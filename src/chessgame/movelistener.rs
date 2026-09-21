use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MoveState{
    ActiveSquare(i8,i8),
    //ActivePiece(i8,i8),
    //Promotion(i8,i8,i8,i8),
    #[default]
    None
}

#[derive(Event)]
pub struct SquareClicked{
    pub square:(i8,i8)
}

pub fn click_listener(
    click : On<SquareClicked>,
    state: Res<State<MoveState>>,
    mut next_state: ResMut<NextState<MoveState>>,
    
){
    let square = click.square;
    match state.get(){
        MoveState::ActiveSquare(x,y) => {
            println!("make move {} {} to {} {}", x,y, square.0,square.1);
            next_state.set(MoveState::None);
        }
        MoveState::None => {
            next_state.set(MoveState::ActiveSquare(square.0, square.1));
        }

    }

}
use bevy::prelude::*;
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum MoveState{
    ActiveSquare(i8,i8),
    ActivePiece(i8,i8),
    Promotion(i8,i8,i8,i8),
    #[default]
    None
}

#[derive(Resource,Default)]
pub struct MoveListener{
    active_square: Option<(i8,i8)>,
}

#[derive(Event)]
pub struct SquareClicked{
    pub square:(i8,i8)
}

pub fn click_listener(
    click : On<SquareClicked>,
    mut move_listener : ResMut<MoveListener>
    
){
    let square = click.square;
    match move_listener.active_square{
        Some(v) => {
            println!("make move {} {} to {} {}", v.0,v.1,square.0,square.1);
            move_listener.active_square = None;
        },
        None => {
            move_listener.active_square = Some(square);
        }

    }

}
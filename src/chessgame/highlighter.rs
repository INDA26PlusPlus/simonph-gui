use bevy::prelude::*;
use super::movelistener::MoveListener;
use super::board::Square;
use super::board::BoardPosition;
use super::movevalidator::MetaBoard;
use super::boardconstants::*;
#[derive(Component)]
pub struct SquareHighlight{}

pub fn highlight_square(
    move_state: Res<MoveListener>, 
    mut commands: Commands, 
    query:Query<(Entity,&BoardPosition),With<Square>>){
    let square = match move_state.start{
        Some(v) => v,
        _ => return,
    };
    for (entity, pos) in query{
        if pos.x != square.0 || pos.y != square.1{
            continue;
        }
        commands.entity(entity).insert(SquareHighlight{});
    }
}
pub fn added_active(
    query: Query<&MeshMaterial2d<ColorMaterial>, Added<SquareHighlight>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for material in &query {
        if let Some(mut material) = materials.get_mut(&material.0) {
            material.color = Color::srgb(1.0, 1.0, 0.0);
        }
    }
}
pub fn highlight_legal_moves(move_state: Res<MoveListener>, 
    mut commands: Commands, 
    query:Query<(Entity,&BoardPosition),With<Square>>,
    meta_board:Res<MetaBoard>)

{
    println!("gaah");
    let square = match move_state.start{
        Some(v)=> v,
        _ => return,
    };
    let legal_moves = meta_board.get_legal_moves(square);
    for x in &legal_moves{
        println!("{} {}",x.0,x.1);
    }
    for (entity, pos) in query{
        if legal_moves.contains(&(pos.x,pos.y)){
            commands.entity(entity).insert(SquareHighlight{});
        }
        
    }
}
pub fn remove_active(
    query: Query<(&MeshMaterial2d<ColorMaterial>,&BoardPosition,Entity),With<SquareHighlight>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
){
    for (material,pos,entity) in &query{
        let tilecolour = match (pos.x+pos.y)%2{
            1 => WHITE_COLOUR.clone(),
            0 => BLACK_COLOUR.clone(),
            _ => panic!("aah"),
        };
        if let Some(mut material) = materials.get_mut(&material.0) {
            material.color = tilecolour;
        }
        commands.entity(entity).remove::<SquareHighlight>();
    }
}
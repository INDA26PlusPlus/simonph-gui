use bevy::prelude::*;
use samolss_chess::board::Piece;
use super::boardconstants::*;
use crate::chessgame::movelistener::MoveListener;
use crate::chessgame::movevalidator::MetaBoard;
use super::follower::FollowMouse;
use super::chessassets::PieceAssets;
use super::board::BoardPosition;
use super::event::BoardUpdated;
use super::board::get_world_position;
#[derive(Component)]
pub struct PieceComponent{}

pub fn update_piece_sprite(_board_updated:On<BoardUpdated>,
    pieces:Query<(&mut Sprite, &BoardPosition,&mut Visibility, &mut Transform),With<PieceComponent>>,
    images: Res<Assets<Image>>,
    assets:Res<PieceAssets>,
    board:Res<MetaBoard>,
    
){
    for (mut sprite, pos,mut vis,mut transform) in pieces{
        let piece_type:Piece = board.get_piece((pos.x,pos.y));
        match assets.get_piece_asset(&piece_type){
            None => {
                *vis = Visibility::Hidden;
            },
            Some(img) => {
                let Some(img_asset) = images.get(&img) else{
                    continue;
                };
                *vis = Visibility::Visible;
                sprite.image = img;
                let w = PIECE_WIDTH*(img_asset.width() as f32);
                let ratio = img_asset.height() as f32 / img_asset.width() as f32;
                let h = w*ratio;
                sprite.custom_size = Some(Vec2::new(w,h));
                let (posx, mut posy) = get_world_position(pos.x, pos.y);
                posy += h/2.0 + PIECE_LOWER - TILE_SIZE/2.0;
                *transform = Transform::from_xyz(posx,posy,1.0);
            },
        };
        
    }
}
pub fn getposition(img:Handle<Image>,images: &Assets<Image>, pos:(usize,usize)) -> ((f32,f32),(f32,f32)){
    let Some(img_asset) = images.get(&img) else{
        panic!("graaah");
    };
    let w = PIECE_WIDTH*(img_asset.width() as f32);
    let ratio = img_asset.height() as f32 / img_asset.width() as f32;
    let h = w*ratio;
    let (posx, mut posy) = get_world_position(pos.0, pos.1);
    posy += h/2.0 + PIECE_LOWER - TILE_SIZE/2.0;
    return ((w,h),(posx,posy));
}
pub fn set_active_follow(
    query:Query<(Entity,&BoardPosition),With<PieceComponent>>,
    mut commands:Commands,
    state:Res<MoveListener>
){
    let piece_cords = match state.start{
        None => return,
        Some(v) => v,
    };
    for (entity,pos) in query{
        if pos.x != piece_cords.0 || pos.y != piece_cords.1{
            continue;
        }
        commands.entity(entity).insert(FollowMouse);
    }
}
pub fn remove_active_follow(
    query:Query<Entity,(With<PieceComponent>,With<FollowMouse>)>,
    mut commands:Commands
){
    for entity in query{
        commands.entity(entity).remove::<FollowMouse>();
    }
    commands.trigger(BoardUpdated{});
}

pub fn trigger_board_updated(mut commands:Commands){
    commands.trigger(BoardUpdated{});
}
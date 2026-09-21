use bevy::prelude::*;
use samolss_chess::board::Piece;
use super::boardconstants::*;
use crate::chessgame::movevalidator::MetaBoard;

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
                println!("{ratio}");
                let h = w*ratio;
                sprite.custom_size = Some(Vec2::new(w,h));
                let (posx, mut posy) = get_world_position(pos.x, pos.y);
                posy += h/2.0 + PIECE_LOWER - TILE_SIZE/2.0;
                *transform = Transform::from_xyz(posx,posy,1.0);
            },
        };
        
    }
}
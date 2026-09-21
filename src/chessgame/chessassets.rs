use bevy::prelude::*;
use samolss_chess::board::Piece;
#[derive(Resource)]
pub struct PieceAssets {
    pub white_pawn: Handle<Image>,
    pub white_knight: Handle<Image>,
    pub white_bishop: Handle<Image>,
    pub white_rook: Handle<Image>,
    pub white_queen: Handle<Image>,
    pub white_king: Handle<Image>,

    pub black_pawn: Handle<Image>,
    pub black_knight: Handle<Image>,
    pub black_bishop: Handle<Image>,
    pub black_rook: Handle<Image>,
    pub black_queen: Handle<Image>,
    pub black_king: Handle<Image>,
}

impl PieceAssets {
    pub fn get_piece_asset(&self, piece: &Piece) -> Option<Handle<Image>> {
        match piece {
            Piece::Empty => None,
            Piece::Pawn { is_white, .. } => {
                if *is_white {
                    Some(self.white_pawn.clone())
                } else {
                    Some(self.black_pawn.clone())
                }
            }
            Piece::Rook { is_white, .. } => {
                if *is_white {
                    Some(self.white_rook.clone())
                } else {
                    Some(self.black_rook.clone())
                }
            }
            Piece::Knight { is_white } => {
                if *is_white {
                    Some(self.white_knight.clone())
                } else {
                    Some(self.black_knight.clone())
                }
            }
            Piece::Bishop { is_white } => {
                if *is_white {
                    Some(self.white_bishop.clone())
                } else {
                    Some(self.black_bishop.clone())
                }
            }
            Piece::Queen { is_white } => {
                if *is_white {
                    Some(self.white_queen.clone())
                } else {
                    Some(self.black_queen.clone())
                }
            }
            Piece::King { is_white, .. } => {
                if *is_white {
                    Some(self.white_king.clone())
                } else {
                    Some(self.black_king.clone())
                }
            }
        }
    }
}

pub fn load_piece_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PieceAssets {
        white_pawn: asset_server.load("WhitePieces/pawn.png"),
        white_knight: asset_server.load("WhitePieces/knight.png"),
        white_bishop: asset_server.load("WhitePieces/bishop.png"),
        white_rook: asset_server.load("WhitePieces/rook.png"),
        white_queen: asset_server.load("WhitePieces/queen.png"),
        white_king: asset_server.load("WhitePieces/king.png"),
        black_pawn: asset_server.load("BlackPieces/pawn.png"),
        black_bishop: asset_server.load("BlackPieces/bishop.png"),
        black_knight: asset_server.load("BlackPieces/knight.png"),
        black_rook: asset_server.load("BlackPieces/rook.png"),
        black_queen: asset_server.load("BlackPieces/queen.png"),
        black_king: asset_server.load("BlackPieces/king.png"),
    });
}
use bevy::prelude::*;
mod board;
mod movelistener;
mod movevalidator;
mod chessassets;
mod piece;
mod event;
mod boardconstants;
use crate::chessgame::board::remove_active;

use super::GameState;
pub use event::MakeBoard;
pub struct ChessGamePlugin;
impl Plugin for ChessGamePlugin{
    fn build(&self, app: &mut App) {
        app.init_state::<movelistener::MoveState>()
        .add_systems(Startup, chessassets::load_piece_assets)
        .add_observer(movelistener::click_listener)
        .add_systems(Update, board::click_square)
        .init_resource::<movevalidator::MetaBoard>()
        .add_observer(piece::update_piece_sprite)
        .add_systems(OnEnter(GameState::PlayingGame),board::makeboard)
        .add_observer(board::highlight_square)
        .add_systems(Update,board::added_active)
        .add_systems(OnEnter(movelistener::MoveState::None),remove_active)
        .add_observer(board::highlight_legal_moves);
    }
}
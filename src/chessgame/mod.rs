use bevy::prelude::*;
mod board;
mod movelistener;
mod movevalidator;
mod chessassets;
mod piece;
mod event;
mod boardconstants;
pub struct ChessGamePlugin;
impl Plugin for ChessGamePlugin{
    fn build(&self, app: &mut App) {
        app.init_state::<movelistener::MoveState>()
        .add_systems(Startup, chessassets::load_piece_assets)
        .add_observer(movelistener::click_listener)
        .add_systems(Startup, board::makeboard)
        .add_systems(Update, board::click_square)
        .init_resource::<movevalidator::MetaBoard>()
        .add_observer(piece::update_piece_sprite);
    }
}
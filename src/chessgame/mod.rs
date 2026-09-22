use bevy::prelude::*;
mod board;
mod movelistener;
mod movevalidator;
mod chessassets;
mod piece;
mod event;
mod boardconstants;
mod states;
use states::*;
mod highlighter;

use super::GameState;
pub use event::MakeBoard;
pub struct ChessGamePlugin;
impl Plugin for ChessGamePlugin{
    fn build(&self, app: &mut App) {
        app.init_state::<MoveState>()
        .init_state::<HighlightStage>()
        .add_observer(set_clear_observer)
        .add_systems(OnExit(MoveState::ActiveSquare),set_clear)
        .add_systems(OnEnter(HighlightStage::Clear), redirect_clear)

        .add_observer(set_none)
        .add_observer(set_active_square)
        .add_observer(piece::update_piece_sprite)

        .init_resource::<movevalidator::MetaBoard>()

        .init_resource::<movelistener::MoveListener>()
        .add_observer(movelistener::none_click_listener.run_if(in_state(MoveState::None)))
        .add_observer(movelistener::has_active_listener.run_if(in_state(MoveState::ActiveSquare)))
        .add_observer(movelistener::update_start)
        .add_observer(movelistener::reset_move_listener)

        .add_systems(OnEnter(HighlightStage::Clear),highlighter::remove_active)
        .add_systems(OnEnter(HighlightStage::Active),highlighter::highlight_legal_moves)
        .add_systems(OnEnter(HighlightStage::Active), highlighter::highlight_square)
        .add_systems(Update,highlighter::added_active)

        .add_systems(Startup,chessassets::load_piece_assets)
        .add_systems(OnEnter(GameState::PlayingGame),board::makeboard)
        .add_systems(Update,board::click_square);
    }
}
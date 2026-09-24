use bevy::prelude::*;
mod board;
mod movelistener;
mod movevalidator;
mod chessassets;
mod piece;
mod event;
mod boardconstants;
pub mod states;
mod follower;
use states::*;
mod highlighter;
mod gameoverhandler;
mod promotion;
use crate::chessgame::piece::trigger_board_updated;

use super::GameState;
pub struct ChessGamePlugin;
impl Plugin for ChessGamePlugin{
    fn build(&self, app: &mut App) {
        app.init_state::<MoveState>()
        .init_state::<HighlightStage>()
        .add_observer(set_clear_observer)
        .add_systems(OnEnter(MoveState::None),set_clear)
        .add_systems(OnEnter(MoveState::Off),set_clear)
        .add_systems(OnEnter(MoveState::Promotion),set_clear)
        .add_systems(OnEnter(HighlightStage::Clear), redirect_clear)

        .init_state::<BoardState>()
        .add_systems(OnEnter(BoardState::Startup),set_playing)
        .add_systems(OnEnter(GameState::PlayingGame), set_startup)
        .add_systems(OnExit(GameState::PlayingGame), set_hidden)

        .add_observer(set_none)
        .add_observer(set_active_square)
        .add_observer(set_promotion)
        .add_systems(OnExit(BoardState::Playing),turn_off)
        .add_systems(OnEnter(BoardState::Playing),activate_movestate)

        .add_observer(piece::update_piece_sprite)
        .add_systems(OnExit(BoardState::Startup),trigger_board_updated)
        .add_systems(OnEnter(MoveState::ActivePiece), piece::set_active_follow)
        .add_systems(OnExit(MoveState::ActivePiece), piece::remove_active_follow)
        .add_systems(OnEnter(MoveState::Promotion),piece::hideoverlapping)

        .init_resource::<movevalidator::MetaBoard>()
        .add_systems(OnEnter(BoardState::Startup),movevalidator::reset_board)

        .init_resource::<movelistener::MoveListener>()
        .add_observer(movelistener::none_click_listener.run_if(in_state(MoveState::None)))
        .add_observer(movelistener::has_active_listener.run_if(in_state(MoveState::ActiveSquare)))
        .add_observer(movelistener::has_active_piece_listener.run_if(in_state(MoveState::ActivePiece)))
        .add_observer(movelistener::has_promotion.run_if(in_state(MoveState::Promotion)))
        .add_observer(movelistener::update_start)
        .add_observer(movelistener::reset_move_listener)
        .add_observer(movelistener::update_end)

        .add_systems(OnEnter(HighlightStage::Clear),highlighter::remove_active)
        .add_systems(OnEnter(HighlightStage::Active),highlighter::highlight_legal_moves)
        .add_systems(OnEnter(HighlightStage::Active), highlighter::highlight_square)
        .add_systems(Update,highlighter::added_active)

        .add_systems(OnEnter(GameState::LoadingResources),chessassets::load_piece_assets)

        .add_systems(OnExit(GameState::LoadingResources),board::makeboard)
        .add_systems(Update,board::click_square.run_if(in_state(MoveState::None).or_else(in_state(MoveState::ActiveSquare))))
        .add_systems(Update,board::drop_square.run_if(in_state(MoveState::ActivePiece)))
        .add_systems(OnExit(BoardState::Hidden), board::showboard)
        .add_systems(OnEnter(BoardState::Hidden), board::hideboard)
        
        .add_systems(Update, follower::follow_mouse)
        
        .add_observer(gameoverhandler::check_checkmate)

        .add_systems(OnEnter(MoveState::Promotion),promotion::spawn_promotion)
        .add_systems(Update,promotion::click_promotion.run_if(in_state(MoveState::Promotion)))
        .add_systems(OnExit(MoveState::Promotion),promotion::remove_promotion);
    }
}
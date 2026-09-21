use bevy::prelude::*;
mod board;
mod movelistener;
mod movevalidator;
pub struct ChessGamePlugin;
impl Plugin for ChessGamePlugin{
    fn build(&self, app: &mut App) {
        app.init_resource::<movelistener::MoveListener>()
        .add_observer(movelistener::click_listener)
        .add_systems(Startup, board::makeboard)
        .add_systems(Update, board::click_square);
    }
}
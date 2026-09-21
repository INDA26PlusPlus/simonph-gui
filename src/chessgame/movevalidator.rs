use samolss_chess::board::Board;
use samolss_chess::board::create_board;
use samolss_chess::move_piece::move_piece;
use bevy::prelude::*;
#[derive(Resource)]
pub struct MetaBoard{
    board:Board,
}
impl MetaBoard{
    pub fn from_fen(fen_string:String)-> MetaBoard{
        MetaBoard{board:create_board(fen_string)}
    }
    pub fn default() -> MetaBoard{
        MetaBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string())
    }
    pub fn make_move(&mut self, start:(usize,usize), end:(usize,usize),promotion_piece:char)->Result<(),()>{
        self.board = match move_piece(self.board.clone(),start,end,promotion_piece){
            Ok(v) => v,
            Err(e) => {
                println!("{e}");
                return Err(())
            }
        };
        return Ok(());
    }
}
impl Default for MetaBoard{
    fn default() -> Self {
        MetaBoard::default()
    }
}
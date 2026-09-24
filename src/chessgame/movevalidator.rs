use samolss_chess::board::Board;
use samolss_chess::board::create_board;
use samolss_chess::move_piece::move_piece;
use samolss_chess::move_piece::gen_all_moves;
pub use samolss_chess::board::Piece;
use bevy::prelude::*;
#[derive(Resource)]
pub struct MetaBoard{
    board:Board,
    legal_moves:Vec<((usize,usize),(usize,usize),char)>,
}
impl MetaBoard{
    pub fn from_fen(fen_string:String)-> MetaBoard{
        let board = create_board(fen_string);
        let legal_moves = gen_all_moves(&board);
        MetaBoard{board,legal_moves}
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
        self.legal_moves = gen_all_moves(&self.board);
        return Ok(());
    }
    pub fn get_piece(&self, pos:(usize,usize))->Piece{
        self.board.squares[pos.1][pos.0]
    }
    pub fn is_same_colour_piece(&self, pos:(usize,usize))-> bool{
        match self.board.squares[pos.1][pos.0].is_white(){
            None => false,
            Some(v) => !self.board.white_turn^v,
        }
    }
    // pub fn get_all_legal_moves(&self)-> Vec<((usize,usize),(usize,usize),char)>{
    //     return gen_all_moves(&self.board);
    // }
    pub fn get_legal_moves(&self, pos:(usize,usize))->Vec<(usize,usize)>{
        let mut ret: Vec<(usize, usize)> = Vec::new();
        for ((sx,sy),(ex,ey),_) in &self.legal_moves{
            if *sx == pos.0 && *sy == pos.1{
                ret.push((*ex,*ey));
            }
        }
        return ret;
    }
    pub fn is_checkmate(&self)->bool{
        return self.legal_moves.len() == 0;
    }
    pub fn pick_promotion(&self,start:(usize,usize), end:(usize,usize))->bool{
        if !self.get_legal_moves(start).contains(&end){
            return false;
        }
        match self.get_piece(start){
            Piece::Pawn {..} => {},
            _ => return false,
        }
        return (end.1 == 7) || (end.1 == 0);
    }
}
impl Default for MetaBoard{
    fn default() -> Self {
        MetaBoard::default()
    }
}
pub fn reset_board(mut meta_board:ResMut<MetaBoard>){
    *meta_board = MetaBoard::default();
}
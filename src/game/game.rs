use crate::game::board::{ Board, PlayerEnum };
use crate::game::errors::TicTacToeError;
use crate::game::player;
use crate::game::player::Player;


pub struct Game {
    board: Board,
    current_turn: PlayerEnum
}

impl Game {

    pub fn new() -> Game {
        Game{
            board: Board::new(),
            current_turn: PlayerEnum::X
        }
    }

    // pub fn play_turn<P: Player>(&mut self, player: &P) {
    //     player.assign_piece(self.current_turn);
    //     let index = player.play_turn(self.board);
    //     match self.board.set(index, self.current_turn) {
    //         Ok(_) => (),
    //         Err(e) => player.on_error()
    //     }
    // }
}

pub fn play<P: Player>(players: Vec<&P>) {
}
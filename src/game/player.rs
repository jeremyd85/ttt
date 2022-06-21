use crate::game::board::{ Board, PlayerEnum };

pub trait Player {
    fn assign_piece(&self, player_enum: PlayerEnum) {}

    fn play_turn(&self, board: Board) -> usize {
        0
    }
}
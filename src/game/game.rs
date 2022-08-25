use crate::game::errors::TicTacToeError;
use crate::game::game_state::{GameState, PlayerEnum};
use crate::game::player;
use crate::game::player::Player;
use std::borrow::Borrow;
use std::ops::Deref;

pub struct Game {
    game_state: GameState,
    player1: Box<dyn Player>,
    player2: Box<dyn Player>,
    turn: PlayerEnum,
}

impl Game {
    pub fn new<P: 'static + Player>(player1: Box<P>, player2: Box<P>) -> Game {
        Game {
            game_state: GameState::new(),
            player1,
            player2,
            turn: PlayerEnum::X,
        }
    }

    fn play_turn(&mut self, turn: PlayerEnum) {
        let mut player = match turn {
            PlayerEnum::X => self.player1.as_mut(),
            PlayerEnum::O => self.player2.as_mut(),
            _ => panic!(),
        };
        let retries = 5;
        for i in 0..retries {
            player.assign_piece(self.turn);
            let index = player.play_turn(self.game_state.clone());
            let valid_moves = self.game_state.empty_positions();
            if !valid_moves.contains(&index) {
                player.on_error(TicTacToeError::IllegalMove, i)
            } else {
                match self.game_state.set(index, self.turn) {
                    Ok(_) => return,
                    Err(e) => player.on_error(e, i),
                }
            }
        }
    }

    pub fn play(&mut self) -> Option<PlayerEnum> {
        loop {
            self.play_turn(self.turn);
            if self.game_state.is_winner(PlayerEnum::X) {
                return Some(PlayerEnum::X);
            } else if self.game_state.is_winner(PlayerEnum::O) {
                return Some(PlayerEnum::O);
            } else if self.game_state.is_tie() {
                return None;
            }
            self.turn = match self.turn {
                PlayerEnum::None => panic!(),
                PlayerEnum::X => PlayerEnum::O,
                PlayerEnum::O => PlayerEnum::X,
            }
        }
    }
}

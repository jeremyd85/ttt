use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Range;
use itertools::Itertools;
use itertools::structs::Permutations;
use crate::game::board::{ Board, PlayerEnum };
use crate::game::consts::BOARD_SIZE;



pub struct BoardIterator {
    board: Board,
    iteration_count: usize,
    permutations: Permutations<Range<usize>>,
    current_turn_order: Option<Vec<usize>>,
    turn_index: usize
}

impl BoardIterator {
    pub fn new() -> BoardIterator {
        BoardIterator{
            board: Board::new(),
            iteration_count: 0,
            permutations: (0..9).permutations(9),
            current_turn_order: None,
            turn_index: 0
        }
    }
}

impl Iterator for BoardIterator {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iteration_count == 0 {
            self.iteration_count += 1;
            return Some(Board::new());
        }
        if self.turn_index >= BOARD_SIZE {
            self.current_turn_order = None
        }
        let mut turn_order = self.current_turn_order.as_ref();
        if turn_order.is_none() {
            self.turn_index = 0;
            self.board = Board::new();
            self.current_turn_order = self.permutations.next();
            turn_order = self.current_turn_order.as_ref();
            if turn_order.is_none() {
                return None;
            }
        }
        let pos = turn_order.unwrap()[self.turn_index];
        let mut player_enum = PlayerEnum::X;
        if self.turn_index % 2 != 0 {
            player_enum = PlayerEnum::O;
        }
        self.board.set(pos, player_enum).unwrap();
        if self.board.is_winner(player_enum) {
            self.current_turn_order = None
        }
        self.turn_index += 1;
        self.iteration_count += 1;
        return Some(self.board)
    }
}
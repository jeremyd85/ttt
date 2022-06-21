mod game;

use std::time::{Duration, Instant};
use itertools::Itertools;
use crate::game::board::{Board, PlayerEnum};
use crate::game::board_iterator::BoardIterator;


fn play_game() {
    let mut b = Board::new();
    let mut turn_count = 0;
    let mut player = PlayerEnum::X;
    let mut line= String::new();
    println!("{}", b.get_display());
    while !(b.is_winner(PlayerEnum::X) || b.is_winner(PlayerEnum::O) || turn_count == 9) {
        println!("Enter move (0-8): ");
        let input = std::io::stdin().read_line(&mut line).unwrap();
        let pos = line.trim().parse::<u32>().unwrap();
        line.clear();
        if pos > 9 || !b.is_empty(pos as usize).unwrap() {
            println!("Invalid move!");
            continue
        }
        b.set(pos as usize, player).unwrap();
        if matches!(player, PlayerEnum::X) {
            player = PlayerEnum::O
        } else {
            player = PlayerEnum::X
        }
        turn_count += 1;
        println!("{}", b.get_display());
    }
    if b.is_winner(PlayerEnum::X) {
        println!("X wins!")
    } else if b.is_winner(PlayerEnum::O) {
        println!("O wins!")
    } else {
        println!("Tie :(")
    }
}

pub fn get_true_board(board: Board) -> Board {
    let num_transformations = 8;
    let mut min_board = board.clone();
    for i in 0..num_transformations {
        let transformed_board = board.transform(i % 4, i > 3);
        if transformed_board.get_raw() < min_board.get_raw() {
            min_board = transformed_board
        }
    }
    return min_board
}


fn main() {
    let boards = BoardIterator::new();
    let unique_boards = boards.map(|b| get_true_board(b)).unique().collect_vec();
    let o_winner_boards = unique_boards.iter().filter(|b| b.is_winner(PlayerEnum::O)).collect_vec();
    let x_winner_boards = unique_boards.iter().filter(|b| b.is_winner(PlayerEnum::X)).collect_vec();
    let tie_boards = unique_boards.iter().filter(|b| b.is_tie()).collect_vec();
    println!("Total: {}", unique_boards.len());
    println!("O winners: {}", o_winner_boards.len());
    println!("X winners: {}", x_winner_boards.len());
    println!("Ties: {}", tie_boards.len());
    let endings = o_winner_boards.len() + x_winner_boards.len() + tie_boards.len();
    println!("Endings: {}", endings);
    let incomplete = unique_boards.len() - endings;
    println!("Incomplete: {}", incomplete);
    for tie_board in tie_boards {
        println!("{}\n", tie_board.get_display());
    }
}

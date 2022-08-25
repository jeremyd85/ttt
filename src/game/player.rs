use crate::game::errors::TicTacToeError;
use crate::game::game_state::{GameState, PlayerEnum};
use crate::{play_game, Game};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::io::Write;

pub trait Player {
    fn assign_piece(&mut self, player_enum: PlayerEnum) {}

    fn play_turn(&mut self, board: GameState) -> usize {
        0
    }
    fn on_error(&mut self, error: TicTacToeError, retries: i32) {}
}

pub struct HumanPlayer {
    player_enum: PlayerEnum,
}

impl HumanPlayer {
    pub fn new() -> HumanPlayer {
        HumanPlayer {
            player_enum: PlayerEnum::None,
        }
    }
}

impl Player for HumanPlayer {
    fn assign_piece(&mut self, player_enum: PlayerEnum) {
        self.player_enum = player_enum
    }

    fn play_turn(&mut self, board: GameState) -> usize {
        let d = board.get_numbered_display();
        println!("{}", d);
        let mut line = String::new();
        print!("Enter move (1-9): ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        let pos = line.trim().parse::<usize>().unwrap();
        line.clear();
        pos - 1
    }

    fn on_error(&mut self, error: TicTacToeError, retries: i32) {
        match error {
            TicTacToeError::InvalidBoard => {}
            TicTacToeError::InvalidPlayerEnum => {}
            TicTacToeError::IllegalMove => println!("That space is taken!"),
            TicTacToeError::OutOfBounds => {
                println!("You must enter a value in between 1 and 9 for input")
            }
        }
        println!("Number of retries left: {}", retries)
    }
}

pub fn simplified_board(board: GameState) -> GameState {
    let num_transformations = 8;
    let mut min_board = board.clone();
    for i in 0..num_transformations {
        let transformed_board = board.transform(i % 4, i > 3);
        if transformed_board.get_raw() < min_board.get_raw() {
            min_board = transformed_board
        }
    }
    return min_board;
}

pub struct BoardNode {
    game_state: GameState,
    parent: Option<u32>,
}

impl BoardNode {
    pub fn new(parent: Option<u32>, pos: usize) -> BoardNode {
        if let Some(p) = parent {
            let mut parent_gs = GameState::try_from(p).unwrap();
            parent_gs.auto_set(pos).unwrap();
            return BoardNode {
                game_state: parent_gs,
                parent: parent,
            };
        }
        BoardNode {
            game_state: GameState::new(),
            parent: None,
        }
    }

    pub fn children(&self) -> Vec<GameState> {
        let mut children = vec![];
        for pos in self.game_state.empty_positions() {
            let mut child = self.game_state.clone();
            child.auto_set(pos).unwrap();
            children.push(child);
        }
        children
    }

    pub fn unique_children(&self) -> Vec<GameState> {
        let mut children = HashSet::new();
        for child in self.children() {
            let simplified_child = simplified_board(child);
            children.insert(simplified_child);
        }
        children.iter().map(|c| *c).collect_vec()
    }

    pub fn parent_board(&self) -> Option<GameState> {
        if let Some(p) = self.parent {
            return Some(GameState::try_from(p).unwrap());
        }
        return None;
    }
}

struct AIPlayer {
    player_enum: PlayerEnum,
    graph: HashMap<u32, BoardNode>,
}

impl AIPlayer {
    fn score_board(board: GameState, player_enum: PlayerEnum) -> f32 {
        let opponent_enum = if player_enum == PlayerEnum::X {
            PlayerEnum::O
        } else {
            PlayerEnum::X
        };
        if board.is_winner(player_enum) {
            return 9.0;
        } else if board.is_winner(opponent_enum) {
            return -9.0;
        } else if board.is_tie() {
            return 0.5;
        }
        0.0
    }
}

impl Player for AIPlayer {
    fn assign_piece(&mut self, player_enum: PlayerEnum) {
        self.player_enum = player_enum
    }

    fn play_turn(&mut self, board: GameState) -> usize {
        0
    }

    fn on_error(&mut self, error: TicTacToeError, retries: i32) {}
}

pub fn minmax(node: BoardNode, depth: i32, player_enum: PlayerEnum) -> usize {
    if depth == 0 || node.game_state.
    0
}
use crate::game::consts::{
    BOARD_SIZE, PLAYER_NONE, PLAYER_O, PLAYER_X, POSITION_MAP, TRANSFORM_SHIFTS, WIN_MASKS,
    X_BIT_MASK,
};
use crate::game::errors::TicTacToeError;
use itertools::Itertools;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PlayerEnum {
    None = PLAYER_NONE,
    X = PLAYER_X,
    O = PLAYER_O,
}

impl TryFrom<u32> for PlayerEnum {
    type Error = TicTacToeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PlayerEnum::None),
            1 => Ok(PlayerEnum::X),
            2 => Ok(PlayerEnum::O),
            _ => Err(TicTacToeError::InvalidPlayerEnum),
        }
    }
}

impl FromStr for PlayerEnum {
    type Err = TicTacToeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            " " => Ok(PlayerEnum::None),
            "X" => Ok(PlayerEnum::X),
            "O" => Ok(PlayerEnum::O),
            _ => Err(TicTacToeError::InvalidPlayerEnum),
        }
    }
}

impl fmt::Display for PlayerEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlayerEnum::X => write!(f, "X"),
            PlayerEnum::O => write!(f, "O"),
            PlayerEnum::None => write!(f, " "),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameState {
    raw: u32,
}

// 8 7 4
// 1 0 3
// 2 5 6

impl GameState {
    pub fn new() -> GameState {
        GameState { raw: 0 }
    }

    pub fn set(&mut self, pos: usize, value: PlayerEnum) -> Result<(), TicTacToeError> {
        if pos > BOARD_SIZE - 1 {
            return Err(TicTacToeError::OutOfBounds);
        }
        let actual_pos = POSITION_MAP[pos];
        self.raw |= (value as u32) << (actual_pos * 2);
        Ok(())
    }

    pub fn get(&self, pos: usize) -> Result<PlayerEnum, TicTacToeError> {
        if pos > BOARD_SIZE - 1 {
            return Err(TicTacToeError::OutOfBounds);
        }
        let actual_pos = POSITION_MAP[pos];
        let player_num = self.raw >> (actual_pos * 2) & 3;
        return PlayerEnum::try_from(player_num);
    }

    pub fn get_turn(&self) -> PlayerEnum {
        let mut x_count = 0;
        let mut o_count = 0;
        for i in 0..BOARD_SIZE {
            match self.get(i).unwrap() {
                PlayerEnum::None => (),
                PlayerEnum::X => x_count += 1,
                PlayerEnum::O => o_count += 1,
            }
        }
        let player = if x_count == o_count {
            PlayerEnum::X
        } else {
            PlayerEnum::O
        };
        player
    }
    
    pub fn auto_set(&mut self, pos: usize) -> Result<(), TicTacToeError> {
        self.set(pos, self.get_turn())
    }

    pub fn is_empty(&self, pos: usize) -> Result<bool, TicTacToeError> {
        match self.get(pos)? {
            PlayerEnum::None => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn inverted(&self) -> GameState {
        let mut piece_mask = ((self.raw & 0x000AAAAA) >> 1) | (self.raw & 0x00055555);
        piece_mask |= piece_mask << 1;
        GameState {
            raw: !self.raw & piece_mask,
        }
    }

    pub fn transform(&self, rotations: i32, flip: bool) -> GameState {
        let transform_index = if flip { 4 } else { 0 } + rotations % 4;
        let mut new_raw = 0;
        for (translation, bit_mask) in TRANSFORM_SHIFTS[transform_index as usize].iter() {
            if *translation < 0 {
                new_raw |= (self.raw & *bit_mask) << (translation.abs() * 2);
            } else if *translation > 0 {
                new_raw |= (self.raw & *bit_mask) >> (translation.abs() * 2);
            } else {
                new_raw |= self.raw & *bit_mask;
            }
        }
        GameState { raw: new_raw }
    }

    pub fn as_vec(&self) -> Vec<String> {
        let mut board_vec = Vec::with_capacity(BOARD_SIZE);
        for i in 0..BOARD_SIZE {
            board_vec.push(self.get(i).unwrap().to_string());
        }
        board_vec
    }

    pub fn get_display(&self) -> String {
        let board_chars = self.as_vec();
        format!(
            " {} | {} | {} \n---+---+---\n {} | {} | {} \n---+---+---\n {} | {} | {} ",
            board_chars[0],
            board_chars[1],
            board_chars[2],
            board_chars[3],
            board_chars[4],
            board_chars[5],
            board_chars[6],
            board_chars[7],
            board_chars[8]
        )
    }

    pub fn get_numbered_display(&self) -> String {
        let board_chars = self
            .as_vec()
            .iter()
            .enumerate()
            .map(|(i, s)| {
                if s == " " {
                    return format!("{}", i + 1);
                } else if s == "X" {
                    return "\x1b[93mX\x1b[0m".to_string();
                } else if s == "O" {
                    return "\x1b[93mO\x1b[0m".to_string();
                }
                return s.to_string();
            })
            .collect_vec();
        format!(
            " {} | {} | {} \n---+---+---\n {} | {} | {} \n---+---+---\n {} | {} | {} ",
            board_chars[0],
            board_chars[1],
            board_chars[2],
            board_chars[3],
            board_chars[4],
            board_chars[5],
            board_chars[6],
            board_chars[7],
            board_chars[8]
        )
    }

    pub fn get_raw(&self) -> u32 {
        self.raw
    }

    pub fn is_winner(&self, player: PlayerEnum) -> bool {
        let raw_board = if matches!(player, PlayerEnum::X) {
            self.raw
        } else {
            self.inverted().raw
        };
        WIN_MASKS.iter().any(|&mask| (raw_board & mask) == mask)
    }

    pub fn is_valid(&self) -> bool {
        // Not an invalid number of pieces on the board or multiple winners
        let mut x_count = 0;
        let mut o_count = 0;
        for i in 0..BOARD_SIZE {
            let player_enum = self.get(i);
            match player_enum {
                Ok(p) => match p {
                    PlayerEnum::X => x_count += 1,
                    PlayerEnum::O => o_count += 1,
                    PlayerEnum::None => (),
                },
                Err(_) => return false,
            }
        }
        (x_count == o_count || x_count == o_count + 1)
            && !(self.is_winner(PlayerEnum::X) && self.is_winner(PlayerEnum::O))
    }

    pub fn is_tie(&self) -> bool {
        for i in 0..BOARD_SIZE {
            let player_num = self.raw >> (i * 2) & 3;
            if matches!(PlayerEnum::try_from(player_num).unwrap(), PlayerEnum::None) {
                return false;
            }
        }
        true && !(self.is_winner(PlayerEnum::X) || self.is_winner(PlayerEnum::O))
    }

    pub fn empty_positions(&self) -> Vec<usize> {
        let mut positions = Vec::with_capacity(BOARD_SIZE);
        for i in 0..BOARD_SIZE {
            if self.is_empty(i).unwrap() {
                positions.push(i);
            }
        }
        positions
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b_vec = self.as_vec();
        write!(f, "{}", b_vec.join(""))
    }
}

impl FromStr for GameState {
    type Err = TicTacToeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != BOARD_SIZE {
            return Err(TicTacToeError::InvalidBoard);
        }
        let mut board = GameState::new();
        for (i, ch) in value.chars().enumerate() {
            let player_enum = PlayerEnum::from_str(ch.to_string().as_str())?;
            match player_enum {
                PlayerEnum::X => board.set(i, PlayerEnum::X)?,
                PlayerEnum::O => board.set(i, PlayerEnum::O)?,
                PlayerEnum::None => board.set(i, PlayerEnum::None)?,
            };
        }
        if !board.is_valid() {
            return Err(TicTacToeError::InvalidBoard);
        }
        Ok(board)
    }
}

impl TryFrom<u32> for GameState {
    type Error = TicTacToeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let board = GameState { raw: value };
        if !board.is_valid() {
            return Err(TicTacToeError::InvalidBoard);
        }
        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverted() {
        let test_cases = vec![("XXXOO O  ", "OOOXX X  "), ("XXO X OO ", "OOX O XX ")];
        for (in_board, out_board) in test_cases {
            let board = GameState::from_str(in_board).unwrap();
            let inverted_board = board.inverted();
            assert_eq!(inverted_board.to_string(), out_board)
        }
    }

    #[test]
    fn test_is_winner_x() {
        let test_cases = vec![("XXX O OXO", true), ("XO    O X", false)];
        for (board_str, is_winner) in test_cases {
            let board = GameState::from_str(board_str).unwrap();
            assert_eq!(board.is_winner(PlayerEnum::X), is_winner);
        }
    }

    #[test]
    fn test_is_empty() {
        let test_cases = vec![("         ", 0, true), ("X        ", 0, false)];
        for (board_str, index, is_empty) in test_cases {
            let board = GameState::from_str(board_str).unwrap();
            assert_eq!(board.is_empty(index as usize).unwrap(), is_empty)
        }
    }
    #[test]
    fn test_transform() {
        let test_cases = vec![
            ("X O XOXO ", 0, false, "X O XOXO "),
            ("X O XOXO ", 1, false, "X XOX  OO"),
            ("X O XOXO ", 2, false, " OXOX O X"),
            ("X O XOXO ", 3, false, "OO  XOX X"),
            ("X O XOXO ", 0, true, "XO  XOX O"),
            ("X O XOXO ", 1, true, "X X XOOO "),
            ("X O XOXO ", 2, true, "O XOX  OX"),
            ("X O XOXO ", 3, true, " OOOX X X"),
        ];
        for (input_str, rotation_num, flip, output_str) in test_cases {
            let b = GameState::from_str(input_str).unwrap();
            let rotated_b = b.transform(rotation_num, flip);
            assert_eq!(rotated_b.to_string(), output_str);
        }
    }
    #[test]
    fn test_from_string_to_str() {
        let test_cases = vec![
            ("", "", false),
            ("XXOOO XOX", "XXOOO XOX", true),
            ("XXXOO XOX", "XXXOO XOX", false),
        ];
        for (input_str, expected_str, exists) in test_cases {
            let b = GameState::from_str(input_str);
            match b {
                Ok(b) => {
                    assert!(exists);
                    assert_eq!(b.to_string(), expected_str)
                }
                Err(_) => {
                    assert!(!exists);
                }
            };
        }
    }

    #[test]
    fn test_get() {
        let test_cases = vec![
            ("X        ", 0, "X"),
            ("OX       ", 0, "O"),
            ("         ", 1, " "),
            ("XXOOO XOX", 5, " "),
            ("XXOOO XOX", 8, "X"),
        ];
        for (board_str, index, player_str) in test_cases {
            let board = GameState::from_str(board_str).unwrap();
            assert_eq!(
                board.get(index).unwrap(),
                PlayerEnum::from_str(player_str).unwrap()
            );
        }
    }

    #[test]
    fn test_set() {
        let test_cases = vec![
            ("X        ", 3, "O", "X  O     "),
            (" X       ", 0, "O", "OX       "),
            ("         ", 1, " ", "         "),
            ("XXXOO XO ", 8, "O", "XXXOO XOO"),
        ];
        for (input_str, index, player_str, output_str) in test_cases {
            let mut board = GameState::from_str(input_str).unwrap();
            let player_enum = PlayerEnum::from_str(player_str).unwrap();
            board.set(index, player_enum).unwrap();
            assert_eq!(board.to_string(), output_str);
        }
    }

    #[test]
    fn test_empy_positions() {
        let test_cases = vec![("X        ", vec![1, 2, 3, 4, 5, 6, 7, 8])];
        for (input_str, positions) in test_cases {
            let board = GameState::from_str(input_str).unwrap();
            assert_eq!(board.empty_positions(), positions)
        }
    }
}

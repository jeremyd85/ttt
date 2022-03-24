#[repr(u8)]
pub enum PlayerEnum {
    None=0,
    X=1,
    O=2
}


impl PlayerEnum {

    pub fn to_string(&self) -> String {
        match self {
            PlayerEnum::X => "X".to_string(),
            PlayerEnum::O => "O".to_string(),
            PlayerEnum::None => " ".to_string()
        }
    }

}


pub struct Board {
    raw: u32,
}

// 8 7 4
// 1 0 3
// 2 5 6

impl Board {

    const POSITION_MAP: [u8; 9] = [8, 7, 4, 1, 0, 4, 2, 5, 6];

    pub fn new() -> Board {
        Board { raw: 0 }
    }

    pub fn from_raw(raw: u32) -> Board {
        Board { raw }
    }

    pub fn set(&mut self, pos: u8, value: PlayerEnum) -> bool {
        let actual_pos = POSITION_MAP[pos as usize];
        if !self.is_empty(actual_pos) || actual_pos > 8 {
            false;
        }
        self.raw |= (value as u32) << (pos * 2);
        true
    }

    pub fn get(&self, pos: u8) -> PlayerEnum {
        let actual_pos = POSITION_MAP[pos as usize];
        let player = self.raw >> (actual_pos * 2) & 0x00000003;
        if player == PlayerEnum::None as u32 {
            PlayerEnum::None
        } else if player == PlayerEnum::X as u32 {
            PlayerEnum::X
        } else {
            PlayerEnum::O
        }
    }

    pub fn is_empty(&self, pos: u8) -> bool {
        match self.get(pos) {
            PlayerEnum::None => true,
            _ => false
        }
    }

    pub fn inverted(&self) -> Board {
        let mut piece_mask = ((self.raw & 0x000AAAAA) >> 1) | (self.raw & 0x00055555);
        piece_mask |= piece_mask << 1;
        Board::from_raw(!self.raw & piece_mask)
    }

    pub fn rotate(&self, times: i32) -> Board {
        let mut piece_mask = ((self.raw & 0x000AAAAA) >> 1) | (self.raw & 0x00055555);
        piece_mask |= piece_mask << 1;
        let mut board = Board::from_raw(self.raw & piece_mask);
        for _ in 0..times {
            board = board.inverted();
        }
        board
    }

    pub fn get_display(&self) -> String {
        format!(" {} | {} | {} \n---+---+---\n {} | {} | {} \n---+---+---\n {} | {} | {} ",
            self.get(0).to_string(),
            self.get(1).to_string(),
            self.get(2).to_string(),
            self.get(3).to_string(),
            self.get(4).to_string(),
            self.get(5).to_string(),
            self.get(6).to_string(),
            self.get(7).to_string(),
            self.get(8).to_string()
        )
    }

    pub fn to_string(&self) -> String {
        [
            self.get(0).to_string(),
            self.get(1).to_string(),
            self.get(2).to_string(),
            self.get(3).to_string(),
            self.get(4).to_string(),
            self.get(5).to_string(),
            self.get(6).to_string(),
            self.get(7).to_string(),
            self.get(8).to_string()
        ].join("")
    }

    pub fn from_string(s: &str) -> Board {
        let mut board = Board::new();
        for (i, c) in s.chars().enumerate() {
            match c {
                'X' => board.set(i as u8, PlayerEnum::X),
                'O' => board.set(i as u8, PlayerEnum::O),
                _ => true
            };
        }
        board
    }

    pub fn get_raw(&self) -> u32 {
        self.raw
    }

    pub fn set_raw(&mut self, raw: u32) {
        self.raw = raw
    }

    pub fn is_winner(&self, player: PlayerEnum) -> bool {
        let raw_board = if matches!(player, PlayerEnum::X) { self.raw } else { self.inverted().raw };
        let win_masks = [
            0x10101,
            0x01110,
            0x15000,
            0x00540,
            0x00015,
            0x01041,
            0x04104,
            0x10410,
        ];
        win_masks.iter().any(|&mask| (raw_board & 3) == mask)
    }

    pub fn is_valid(&self) -> bool {
        // Not an invalid number of pieces on the board or multiple winners
        let board_str = self.to_string();
        let x_count = board_str.matches(PlayerEnum::X.to_string().as_str()).count();
        let o_count = board_str.matches(PlayerEnum::O.to_string().as_str()).count();
        (x_count == o_count || (x_count + 1) == o_count)
            && (self.is_winner(PlayerEnum::X) == true
            && self.is_winner(PlayerEnum::O) == true)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_winner_x() {
        let winning_boards = [
            "XXX O OXO",
            "XO  X OOX"];
        for board in winning_boards.iter() {
            let board = super::Board::from_string(board);
            assert!(board.is_winner(super::PlayerEnum::X));
        }
    }
}



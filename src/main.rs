use crate::board::PlayerEnum;

mod board;

fn main() {
    let mut b = board::Board::new();
    b.set(0, board::PlayerEnum::O);
    b.set(1, board::PlayerEnum::X);
    b.set(3, board::PlayerEnum::O);
    b.set(4, board::PlayerEnum::O);
    b.set(8, board::PlayerEnum::O);
    println!("{}", b.get_display());
    println!("{}", b.get_raw());
    println!("{}", b.inverted().get_display());
    println!("{}", b.is_winner(PlayerEnum::O));
}

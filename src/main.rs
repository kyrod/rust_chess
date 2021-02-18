mod engine;

use engine::board::Board;
fn main() {
    let mut my_board = Board::default();
    my_board.print();
    my_board.move_piece(String::from("e2e4"));
    my_board.print();
}

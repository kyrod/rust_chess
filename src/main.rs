mod engine;

use engine::board::Board;
fn main() {
    let my_board = Board::default();
    my_board.print();
}

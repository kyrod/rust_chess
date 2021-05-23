mod engine;

use engine::board::Board;
fn main() {
    let mut my_board = Board::default();
    my_board.print();
    my_board.make_move(String::from("e2e4"));
    my_board.print();
    my_board.make_move(String::from("e7e5"));
    my_board.print();
    let my_board = Board::default();

    let moves1 = my_board.generate_moves();
    let mut count1 = 0;
    for m in moves1 {
        count1 += 1;
        println!(
            "{}, ({}, {}), ({}, {})",
            m.piece.as_string(),
            m.start.0,
            m.start.1,
            m.end.0,
            m.start.1
        )
    }
    let moves2 = my_board.recurse_gen_moves();
    let mut count2 = 0;
    for l in moves2 {
        for m in l {
            count2 += 1;
            /* println!(
                "{}, ({}, {}), ({}, {})",
                m.piece.as_string(),
                m.start.0,
                m.start.1,
                m.end.0,
                m.start.1
            ) */
        }
    }
    println!("{} moves for white", count1);
    println!("Results in {} moves for black", count2);
}

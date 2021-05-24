# Rust Chess

This is a very basic chess engine written in Rust.

The engine comes in an importable `engine` package (see main.rs) that allows you to generate and display the board. You can also make moves in Long Algebraic Notation using the `make_move_from_string` function. Here's a short example of how this works:

```
mod engine;
use engine::board::Board;

fn main() {
    let mut my_board = Board::default();
    my_board.print();
    my_board.make_move_from_string(String::from("e2e4"));
    my_board.print();
    my_board.make_move_from_string(String::from("e7e5"));
    my_board.print();
}

```
The above code will make the first move of a very basic King's Pawn game.

## Known Issues

In its current state, the engine is incomplete, and there is some chess functionality that hasn't yet been implemented (pawn capturing, castling, pawn promotion, en passant, Check/Checkmate, etc.). This functionality is currently being added, and this repo will be updated once those rules are implemented.

## Move Generation

This engine allows for a very basic move generation, iterating through all pieces and generating a vector containing all moves from that position. Here's a short example of how to generate moves. 
```
let board = Board::default();
let moves1 = board.generate_moves();
```
The above will generate all legal first moves for white, on a default board. Each move comes in a `Move` format, which looks like the following: 
```
pub struct Move {
    pub piece: Piece,
    pub start: (usize, usize),
    pub end: (usize, usize),
}
```
Move objects like above can be used to make moves directly using the `Board::make_move_from_move()` function.

Move generation up to a certain depth is not yet implemented, but will be added soon.
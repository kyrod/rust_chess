use super::piece::Piece;
use super::piece::Piece::*;
use super::piece::PieceColor;
use super::piece::PieceType;

use std::cmp;

#[derive(Copy, Clone)]
pub struct Board {
    pub squares: [[Piece; 8]; 8],
    pub move_number: u32,
    pub to_move: Turn,
    pub can_castle: [char; 4],
    pub half_move: u32,
    pub en_passant: (char, char),
}

#[derive(Copy, Clone, PartialEq)]
pub enum Turn {
    White,
    Black,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Move {
    pub piece: Piece,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl Turn {
    pub fn to_string(&self) -> &'static str {
        match *self {
            Turn::White => "White",
            Turn::Black => "Black",
        }
    }
    pub fn as_color(&self) -> PieceColor {
        match *self {
            Turn::White => PieceColor::White,
            Turn::Black => PieceColor::Black,
        }
    }
    pub fn opposite_turn(&self) -> Turn {
        match *self {
            Turn::White => Turn::Black,
            Turn::Black => Turn::White,
        }
    }
}

impl Board {
    pub const fn default() -> Board {
        Board {
            squares: [
                [
                    BRook, BKnight, BBishop, BQueen, BKing, BBishop, BKnight, BRook,
                ],
                [BPawn, BPawn, BPawn, BPawn, BPawn, BPawn, BPawn, BPawn],
                [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
                [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
                [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
                [Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank],
                [WPawn, WPawn, WPawn, WPawn, WPawn, WPawn, WPawn, WPawn],
                [
                    WRook, WKnight, WBishop, WQueen, WKing, WBishop, WKnight, WRook,
                ],
            ],
            move_number: 0,
            to_move: Turn::White,
            can_castle: ['K', 'Q', 'k', 'q'],
            en_passant: ('-', '-'),
            half_move: 0,
        }
    }
    pub fn print(&self) {
        println!("Move: {}", self.move_number);
        println!("To move: {}", self.to_move.to_string());
        for c in self.to_string().chars() {
            print!("{}", c)
        }
        println!("---------------");
    }
    pub fn to_string(&self) -> String {
        let mut board_str = "".to_string();
        for row in &self.squares {
            for col in row {
                board_str.push(col.as_char());
            }
            board_str.push('\n');
        }
        return board_str;
    }
    fn get_castling(s: String) -> [char; 4] {
        let mut v = ['-'; 4];
        for x in s.chars() {
            let i = match x {
                'K' => 0,
                'Q' => 1,
                'k' => 2,
                'q' => 3,
                '-' => 4,
                _ => 4,
            };
            if i != 4 {
                v[i] = x;
            }
        }
        return v;
    }

    fn get_en_passant(string: String) -> (char, char) {
        if string == String::from("-") {
            return ('-', '-');
        }
        return (
            string.chars().next().unwrap(),
            string.chars().next().unwrap(),
        );
    }
    pub fn from_fen(string: String) -> Board {
        let mut x = string.split_whitespace();
        let board = match x.next() {
            Some(y) => y.to_string(),
            None => panic!("Invalid FEN"),
        };
        let to_move = match x.next() {
            Some(y) => match y {
                "w" => Turn::White,
                "b" => Turn::Black,
                _ => Turn::White,
            },
            None => panic!("Invalid FEN"),
        };
        let castle = match x.next() {
            Some(y) => Board::get_castling(y.to_string()),
            None => panic!("Invalid FEN"),
        };
        let en_passant = match x.next() {
            Some(y) => Board::get_en_passant(y.to_string()),
            None => panic!("Invalid FEN"),
        };
        let half_move = match x.next() {
            Some(y) => y.to_string().parse::<u32>().unwrap(),
            None => panic!("Invalid FEN"),
        };
        let full_move = match x.next() {
            Some(y) => y.to_string().parse::<u32>().unwrap(),
            None => panic!("Invalid FEN"),
        };
        let pieces = Board::pieces_from_fen(board);
        return Board {
            squares: pieces,
            move_number: full_move,
            to_move: to_move,
            can_castle: castle,
            half_move: half_move,
            en_passant: en_passant,
        };
    }

    pub fn pieces_from_fen(pieces: String) -> [[Piece; 8]; 8] {
        let lines: Vec<&str> = pieces.split('/').collect();
        let mut squares = [[Piece::Blank; 8]; 8];
        for (i, l) in lines.iter().enumerate() {
            println!("{}", l);
            let mut j = 0;
            for c in l.chars() {
                if c.is_digit(8) {
                    for _ in 0..c.to_digit(8).unwrap() {
                        squares[i][j] = Piece::Blank;
                        j += 1;
                    }
                } else {
                    squares[i][j] = Piece::from_char(c);
                    j += 1;
                }
            }
        }
        return squares;
    }

    pub fn to_FEN(self) {
        //TODO: impl
    }

    pub fn square_to_row_col(square_string: Option<&str>) -> (usize, usize) {
        fn to_col(x: Option<&str>) -> usize {
            match x {
                Some(x) => match &*x {
                    "a" => 0,
                    "b" => 1,
                    "c" => 2,
                    "d" => 3,
                    "e" => 4,
                    "f" => 5,
                    "g" => 6,
                    "h" => 7,
                    _ => panic!("Invalid square!"),
                },
                None => panic!("Invalid square!"),
            }
        }

        fn to_row(x: Option<&str>) -> usize {
            match x {
                Some(x) => match &*x {
                    // Reversed because index is at top
                    "1" => 7,
                    "2" => 6,
                    "3" => 5,
                    "4" => 4,
                    "5" => 3,
                    "6" => 2,
                    "7" => 1,
                    "8" => 0,
                    _ => panic!("Invalid square!"),
                },
                None => panic!("Invalid square!"),
            }
        }
        // converts a 2 char board position into a tuple
        match square_string {
            Some(s) => (to_row(s.get(1..2)), to_col(s.get(0..1))),
            None => (0, 0),
        }
    }

    pub fn is_occupied(self, target: (usize, usize)) -> bool {
        return self.squares[target.0][target.1] != Piece::Blank;
    }

    pub fn get_path(start: (usize, usize), target: (usize, usize)) -> Vec<(usize, usize)> {
        let mut path: Vec<(usize, usize)> = Vec::new();
        let ydiff = target.0 as i8 - start.0 as i8;
        let xdiff = target.1 as i8 - start.1 as i8;
        for _ in 0..cmp::max(xdiff.abs(), ydiff.abs()) {
            path.push((
                (start.0 as i8 + ydiff / cmp::max(xdiff.abs(), ydiff.abs())) as usize,
                (start.1 as i8 + xdiff / cmp::max(xdiff.abs(), ydiff.abs())) as usize,
            ))
        }
        return path;
    }
    pub fn piece_in_path(
        self,
        start: (usize, usize),
        target: (usize, usize),
        piece: Piece,
    ) -> bool {
        if piece.as_type() == PieceType::Knight {
            return self.is_occupied(target);
        }
        let target_line = Board::get_path(start, target);
        for pos in target_line {
            if self.is_occupied(pos) {
                return true;
            }
        }
        return false;
    }

    pub fn validate_move(
        self,
        start: (usize, usize),
        target: (usize, usize),
        piece: Piece,
    ) -> bool {
        if piece.as_char() == Piece::Blank.as_char() {
            // Cannot move an empty square
            return false;
        }
        if self.to_move.as_color() != piece.as_color() {
            // Cannot move opponent's piece
            return false;
        }
        if !piece.valid_move(start, target, self.can_castle) {
            return false;
        }
        if self.piece_in_path(start, target, piece) {
            return false;
        }
        return true;
    }
    pub fn increment_move(&mut self) {
        if self.to_move == Turn::Black {
            self.move_number += 1;
        }
        self.to_move = self.to_move.opposite_turn();
    }

    fn update_castling(&mut self, piece: Piece, location: (usize, usize), target: (usize, usize)) {
        //update can_castle
    }

    fn update_en_passant(
        &mut self,
        piece: Piece,
        location: (usize, usize),
        target: (usize, usize),
    ) {
        // Update the en passant square
    }

    fn update_half_move(&mut self, piece_taken: bool) {
        if piece_taken {
            self.half_move = 0;
        } else {
            self.half_move += 1;
        }
    }

    pub fn move_piece(&mut self, piece: Piece, location: (usize, usize), target: (usize, usize)) {
        self.update_half_move(self.squares[target.0][target.1] != Piece::Blank);
        self.squares[target.0][target.1] = piece;
        self.squares[location.0][location.1] = Piece::Blank;
        self.update_castling(piece, location, target);
        self.update_en_passant(piece, location, target);
        self.increment_move();
    }

    pub fn make_move_from_string(&mut self, move_string: String) {
        // Expecting a 4 char string, from original location to target location
        let location = Board::square_to_row_col(move_string.get(0..2));
        let target = Board::square_to_row_col(move_string.get(2..4));
        let piece = self.squares[location.0][location.1];

        if self.validate_move(location, target, piece) {
            self.move_piece(piece, location, target)
        } else {
            panic!("Invalid move!")
        }
    }

    pub fn make_move_from_move(&mut self, _move: Move) {
        let location = _move.start;
        let target = _move.end;
        let piece = _move.piece;

        if self.validate_move(location, target, piece) {
            self.move_piece(piece, location, target)
        } else {
            panic!("Invalid move!")
        }
    }

    pub fn generate_moves(self) -> Vec<Move> {
        // Horrendously inneficient way to generate moves
        let mut v: Vec<Move> = Vec::new();
        for (i, row) in self.squares.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                let start = (i, j);
                let piece = col;
                for (m, row2) in self.squares.iter().enumerate() {
                    for (n, _) in row2.iter().enumerate() {
                        let target = (m, n);
                        if self.validate_move(start, target, *piece) {
                            v.push(Move {
                                piece: *piece,
                                start: start,
                                end: target,
                            });
                        }
                    }
                }
            }
        }
        return v;
    }

    pub fn recurse_gen_moves(self) -> Vec<Vec<Move>> {
        let mut v: Vec<Vec<Move>> = Vec::new();
        for m in self.generate_moves() {
            let mut temp: Board = self.clone();
            temp.make_move_from_move(m);
            v.push(temp.generate_moves());
        }
        return v;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_string() {
        // basic test that checks that the default board prints correctly
        let board = Board::default();
        let board_string = board.to_string();
        assert_eq!(
            board_string,
            "rnbqkbnr\npppppppp\n--------\n--------\n--------\n--------\nPPPPPPPP\nRNBQKBNR\n"
        );
    }

    #[test]
    fn test_move_piece() {
        // test that pieces are able to move
        let mut board = Board::default();
        board.make_move_from_string(String::from("e2e4"));
        assert_eq!(
            board.to_string(),
            "rnbqkbnr\npppppppp\n--------\n--------\n----P---\n--------\nPPPP-PPP\nRNBQKBNR\n"
        );
        board.make_move_from_string(String::from("d7d6"));
        assert_eq!(
            board.to_string(),
            "rnbqkbnr\nppp-pppp\n---p----\n--------\n----P---\n--------\nPPPP-PPP\nRNBQKBNR\n"
        );
    }

    #[test]
    #[should_panic]
    fn test_move_piece_invalid_row() {
        // test that only valid squares are allowed
        let mut board = Board::default();
        board.make_move_from_string(String::from("e2e9"));
    }
    #[test]
    #[should_panic]
    fn test_move_piece_invalid_col() {
        // test that only valid squares are allowed
        let mut board = Board::default();
        board.make_move_from_string(String::from("z2e4"));
    }

    #[test]
    #[should_panic]
    fn test_move_piece_empty_square() {
        // test that only valid squares are allowed
        let mut board = Board::default();
        board.make_move_from_string(String::from("e3e4"));
    }
    #[test]
    #[should_panic]
    fn test_move_piece_opponent_turn() {
        // test that only valid squares are allowed
        let mut board = Board::default();
        board.make_move_from_string(String::from("e7e6"));
    }

    #[test]
    fn test_move_gen() {
        let board = Board::default();

        let moves1 = board.generate_moves();
        assert_eq!(moves1.len(), 20);
        let moves2 = board.recurse_gen_moves();
        let mut count2 = 0;
        for l in moves2 {
            for _ in l {
                count2 += 1;
            }
        }
        assert_eq!(count2, 400);
    }
    #[test]
    fn test_from_fen() {
        let mut board1 = Board::default();

        let board2 = Board::from_fen(String::from(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        ));
        assert_eq!(board1.to_string(), board2.to_string());
        board1.make_move_from_string(String::from("e2e4"));
        let board2 = Board::from_fen(String::from(
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        ));
        assert_eq!(board1.to_string(), board2.to_string());
        board1.make_move_from_string(String::from("c7c5"));
        board1.make_move_from_string(String::from("g1f3"));
        let board2 = Board::from_fen(String::from(
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
        ));
        assert_eq!(board1.to_string(), board2.to_string());
    }
}

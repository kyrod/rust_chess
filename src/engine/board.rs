use super::piece::Piece;
use super::piece::Piece::*;
use super::piece::PieceColor;

#[derive(Copy, Clone)]
pub struct Board {
    pub squares: [[Piece; 8]; 8],
    pub move_number: u32,
    pub to_move: Turn,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Turn {
    White,
    Black,
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
        if piece.valid_move(start, target) {
            return true;
        }
        return false;
    }
    pub fn increment_move(&mut self) {
        if self.to_move == Turn::Black {
            self.move_number += 1;
        }
        self.to_move = self.to_move.opposite_turn();
    }

    pub fn move_piece(&mut self, piece: Piece, location: (usize, usize), target: (usize, usize)) {
        self.squares[target.0][target.1] = piece;
        self.squares[location.0][location.1] = Piece::Blank;
        self.increment_move();
    }

    pub fn make_move(&mut self, move_string: String) {
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
        board.make_move(String::from("e2e4"));
        assert_eq!(
            board.to_string(),
            "rnbqkbnr\npppppppp\n--------\n--------\n----P---\n--------\nPPPP-PPP\nRNBQKBNR\n"
        );
        board.make_move(String::from("d7d6"));
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
        board.make_move(String::from("e2e9"));
    }
    #[test]
    #[should_panic]
    fn test_move_piece_invalid_col() {
        // test that only valid squares are allowed
        let mut board = Board::default();
        board.make_move(String::from("z2e4"));
    }
}

use super::piece::Piece;
use super::piece::Piece::*;

pub struct Board {
    pub squares: [[Piece; 8]; 8],
    pub move_number: u32,
    pub to_move: Turn,
}

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
}

impl Board {
    pub const fn default() -> Board {
        Board {
            squares: [
                [
                    BRook, BKnight, BBishop, BQueen, BKing, BBishop, BKnight, BRook,
                ],
                [BPawn, BPawn, BPawn, BPawn, BPawn, BPawn, BPawn, BPawn],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
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
        )
    }
}

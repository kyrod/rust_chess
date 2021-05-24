#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
    Blank,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PieceColor {
    White,
    Black,
    None,
}

#[derive(Copy, Clone, PartialEq)]
pub struct PieceObj {
    ptype: PieceType,
    pcolor: PieceColor,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    WKing,
    WQueen,
    WRook,
    WKnight,
    WBishop,
    WPawn,
    BKing,
    BQueen,
    BRook,
    BKnight,
    BBishop,
    BPawn,
    Blank,
}

impl PieceColor {
    pub fn as_string(&self) -> String {
        match *self {
            PieceColor::White => "White",
            PieceColor::Black => "Black",
            PieceColor::None => "None",
        }
        .to_string()
    }
}

impl PieceObj {
    pub fn as_type(&self) -> PieceType {
        return self.ptype;
    }
    pub fn as_color(&self) -> PieceColor {
        return self.pcolor;
    }
    pub fn as_string(&self) -> String {
        return (self.as_color().as_string() + " " + &self.as_type().as_string()).to_string();
    }
    pub fn flip(pos: (usize, usize)) -> (usize, usize) {
        // this is ugly, fix this pls
        return ((-(pos.0 as i8) + 7) as usize, pos.1);
    }
    pub fn valid_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        if start == end {
            return false;
        }
        if (self.as_type() == PieceType::Pawn || self.as_type() == PieceType::King)
            && self.as_color() == PieceColor::Black
        {
            // pawn + king moves can depend on color
            return self
                .as_type()
                .valid_move(PieceObj::flip(start), PieceObj::flip(end));
        }
        return self.as_type().valid_move(start, end);
    }
}

impl Piece {
    pub fn as_piece(&self) -> PieceObj {
        // TODO: find a better way to do this?
        match *self {
            Piece::WKing => PieceObj {
                ptype: PieceType::King,
                pcolor: PieceColor::White,
            },
            Piece::WQueen => PieceObj {
                ptype: PieceType::Queen,
                pcolor: PieceColor::White,
            },
            Piece::WRook => PieceObj {
                ptype: PieceType::Rook,
                pcolor: PieceColor::White,
            },
            Piece::WKnight => PieceObj {
                ptype: PieceType::Knight,
                pcolor: PieceColor::White,
            },
            Piece::WBishop => PieceObj {
                ptype: PieceType::Bishop,
                pcolor: PieceColor::White,
            },
            Piece::WPawn => PieceObj {
                ptype: PieceType::Pawn,
                pcolor: PieceColor::White,
            },
            Piece::BKing => PieceObj {
                ptype: PieceType::King,
                pcolor: PieceColor::Black,
            },
            Piece::BQueen => PieceObj {
                ptype: PieceType::Queen,
                pcolor: PieceColor::Black,
            },
            Piece::BRook => PieceObj {
                ptype: PieceType::Rook,
                pcolor: PieceColor::Black,
            },
            Piece::BKnight => PieceObj {
                ptype: PieceType::Knight,
                pcolor: PieceColor::Black,
            },
            Piece::BBishop => PieceObj {
                ptype: PieceType::Bishop,
                pcolor: PieceColor::Black,
            },
            Piece::BPawn => PieceObj {
                ptype: PieceType::Pawn,
                pcolor: PieceColor::Black,
            },
            Piece::Blank => PieceObj {
                ptype: PieceType::Blank,
                pcolor: PieceColor::None,
            },
        }
    }
    pub fn from_char(c: char) -> Piece {
        match c {
            'K' => Piece::WKing,
            'Q' => Piece::WQueen,
            'R' => Piece::WRook,
            'N' => Piece::WKnight,
            'B' => Piece::WBishop,
            'P' => Piece::WPawn,
            'p' => Piece::BPawn,
            'b' => Piece::BBishop,
            'n' => Piece::BKnight,
            'r' => Piece::BRook,
            'q' => Piece::BQueen,
            'k' => Piece::BKing,
            _ => Piece::Blank,
        }
    }
    pub fn as_char(&self) -> char {
        if self.as_piece().as_color() == PieceColor::White {
            return self.as_type().as_upper();
        }
        return self.as_type().as_char();
    }

    pub fn as_string(&self) -> String {
        return self.as_piece().as_string();
    }

    pub fn as_color(&self) -> PieceColor {
        return self.as_piece().as_color();
    }

    pub fn as_type(&self) -> PieceType {
        return self.as_piece().as_type();
    }

    pub fn valid_move(self, start: (usize, usize), end: (usize, usize)) -> bool {
        return self.as_piece().valid_move(start, end);
    }
}

impl PieceType {
    pub fn as_char(&self) -> char {
        match *self {
            PieceType::King => 'k',
            PieceType::Queen => 'q',
            PieceType::Rook => 'r',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Pawn => 'p',
            PieceType::Blank => '-',
        }
    }
    pub fn as_upper(&self) -> char {
        match *self {
            PieceType::King => 'K',
            PieceType::Queen => 'Q',
            PieceType::Rook => 'R',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Pawn => 'P',
            PieceType::Blank => '-',
        }
    }
    pub fn as_string(&self) -> String {
        match *self {
            PieceType::King => "King",
            PieceType::Queen => "Queen",
            PieceType::Rook => "Rook",
            PieceType::Knight => "Knight",
            PieceType::Bishop => "Bishop",
            PieceType::Pawn => "Pawn",
            PieceType::Blank => "None",
        }
        .to_string()
    }

    pub fn valid_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        fn king_valid_move(start: (usize, usize), end: (usize, usize)) -> bool {
            return ((start.0 as i8 - end.0 as i8).abs() <= 1)
                && ((start.1 as i8 - end.1 as i8).abs() <= 1);
        }
        fn queen_valid_move(start: (usize, usize), end: (usize, usize)) -> bool {
            return rook_valid_move(start, end) || bishop_valid_move(start, end);
        }
        fn rook_valid_move(start: (usize, usize), end: (usize, usize)) -> bool {
            return (start.0 == end.0 && start.1 != end.1)
                || (start.0 != end.0 && start.1 == end.1);
        }
        fn knight_valid_move(start: (usize, usize), end: (usize, usize)) -> bool {
            return ((start.0 as i8 - end.0 as i8).abs() == 2
                && (start.1 as i8 - end.1 as i8).abs() == 1)
                || ((start.0 as i8 - end.0 as i8).abs() == 1
                    && (start.1 as i8 - end.1 as i8).abs() == 2);
        }
        fn bishop_valid_move(start: (usize, usize), end: (usize, usize)) -> bool {
            return start.0 != end.0
                && start.1 != end.1
                && (start.0 as i8 - end.0 as i8).abs() == (start.1 as i8 - end.1 as i8).abs();
        }
        fn pawn_valid_move(start: (usize, usize), end: (usize, usize)) -> bool {
            // temporarily ignoring captures
            if start.0 == 6 {
                return start.1 == end.1 && (start.0 - 1 == end.0 || start.0 - 2 == end.0);
            }
            return start.1 == end.1 && start.0 - 1 == end.0;
        }

        match *self {
            PieceType::King => king_valid_move(start, end),
            PieceType::Queen => queen_valid_move(start, end),
            PieceType::Rook => rook_valid_move(start, end),
            PieceType::Knight => knight_valid_move(start, end),
            PieceType::Bishop => bishop_valid_move(start, end),
            PieceType::Pawn => pawn_valid_move(start, end),
            PieceType::Blank => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_piece_to_char() {
        let piece = Piece::BKing;
        assert_eq!(piece.as_char(), 'k');
    }

    #[test]
    fn test_flip() {
        let pos = (0, 0);
        assert_eq!(PieceObj::flip(pos), (7, 0));
        let pos = (7, 0);
        assert_eq!(PieceObj::flip(pos), (0, 0))
    }

    #[test]
    fn test_pawn_move() {
        let piece = Piece::BPawn;
        assert_eq!(piece.valid_move((1, 1), (1, 2)), false); // sideways pawn move
        assert_eq!(piece.valid_move((1, 1), (2, 1)), true); // forward pawn move
        assert_eq!(piece.valid_move((1, 1), (3, 1)), true); // double pawn move
        let piece = Piece::WPawn;
        assert_eq!(piece.valid_move((6, 1), (5, 1)), true); // sideways pawn move
        assert_eq!(piece.valid_move((6, 1), (4, 1)), true); // forward pawn move
        assert_eq!(piece.valid_move((6, 1), (3, 1)), false); // double pawn move
    }

    #[test]
    fn test_rook_move() {
        let piece = Piece::BRook;
        assert_eq!(piece.valid_move((1, 1), (1, 5)), true); // sideways move
        assert_eq!(piece.valid_move((1, 1), (7, 1)), true); // forward move
        assert_eq!(piece.valid_move((7, 1), (1, 1)), true); // backwards move
        assert_eq!(piece.valid_move((7, 1), (6, 2)), false); // diagonal move
        assert_eq!(piece.valid_move((7, 1), (5, 2)), false); // knight move
    }

    #[test]
    fn test_bishop_move() {
        let piece = Piece::BBishop;
        assert_eq!(piece.valid_move((1, 1), (1, 5)), false); // sideways move
        assert_eq!(piece.valid_move((1, 1), (7, 1)), false); // forward move
        assert_eq!(piece.valid_move((7, 1), (1, 1)), false); // backwards move
        assert_eq!(piece.valid_move((5, 5), (6, 4)), true); // diagonal move
        assert_eq!(piece.valid_move((5, 5), (4, 4)), true); // diagonal move
        assert_eq!(piece.valid_move((7, 1), (5, 2)), false); // knight move
    }
    #[test]
    fn test_queen_move() {
        let piece = Piece::BQueen;
        assert_eq!(piece.valid_move((1, 1), (1, 5)), true); // sideways move
        assert_eq!(piece.valid_move((1, 1), (7, 1)), true); // forward move
        assert_eq!(piece.valid_move((7, 1), (1, 1)), true); // backwards move
        assert_eq!(piece.valid_move((5, 5), (6, 4)), true); // diagonal move
        assert_eq!(piece.valid_move((5, 5), (4, 4)), true); // diagonal move
        assert_eq!(piece.valid_move((7, 1), (5, 2)), false); // knight move
    }

    #[test]
    fn test_knight_move() {
        let piece = Piece::BKnight;
        assert_eq!(piece.valid_move((1, 1), (1, 5)), false); // sideways move
        assert_eq!(piece.valid_move((1, 1), (7, 1)), false); // forward move
        assert_eq!(piece.valid_move((7, 1), (1, 1)), false); // backwards move
        assert_eq!(piece.valid_move((5, 5), (6, 4)), false); // diagonal move
        assert_eq!(piece.valid_move((5, 5), (4, 4)), false); // diagonal move
        assert_eq!(piece.valid_move((5, 5), (4, 3)), true); // knight move
        assert_eq!(piece.valid_move((5, 5), (6, 3)), true); // knight move
        assert_eq!(piece.valid_move((5, 5), (6, 7)), true); // knight move
        assert_eq!(piece.valid_move((5, 5), (4, 7)), true); // knight move
        assert_eq!(piece.valid_move((5, 5), (7, 4)), true); // knight move
        assert_eq!(piece.valid_move((5, 5), (7, 6)), true); // knight move
        assert_eq!(piece.valid_move((5, 5), (3, 4)), true); // knight move
        assert_eq!(piece.valid_move((5, 5), (3, 6)), true); // knight move
    }
    #[test]
    fn test_king_move() {
        let piece = Piece::BKing;
        assert_eq!(piece.valid_move((1, 1), (1, 5)), false); // sideways move
        assert_eq!(piece.valid_move((1, 1), (7, 1)), false); // forward move
        assert_eq!(piece.valid_move((7, 1), (1, 1)), false); // backwards move
        assert_eq!(piece.valid_move((5, 5), (7, 7)), false); // diagonal move
        assert_eq!(piece.valid_move((5, 5), (3, 7)), false); // diagonal move
        assert_eq!(piece.valid_move((5, 5), (4, 3)), false); // knight move
        assert_eq!(piece.valid_move((5, 5), (6, 3)), false); // knight move
        assert_eq!(piece.valid_move((5, 5), (4, 5)), true); // 1 space
        assert_eq!(piece.valid_move((5, 5), (4, 4)), true); // 1 space
        assert_eq!(piece.valid_move((5, 5), (5, 4)), true); // 1 space
    }
}

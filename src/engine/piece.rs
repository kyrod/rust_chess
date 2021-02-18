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
    None,
}

impl Piece {
    pub fn as_char(&self) -> char {
        match *self {
            Piece::WKing => 'K',
            Piece::WQueen => 'Q',
            Piece::WRook => 'R',
            Piece::WKnight => 'N',
            Piece::WBishop => 'B',
            Piece::WPawn => 'P',
            Piece::BKing => 'k',
            Piece::BQueen => 'q',
            Piece::BRook => 'r',
            Piece::BKnight => 'n',
            Piece::BBishop => 'b',
            Piece::BPawn => 'p',
            Piece::None => '-',
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_piece_to_char() {
        let piece = Piece::BKing;
        assert_eq!(piece.as_char(), 'k')
    }
}

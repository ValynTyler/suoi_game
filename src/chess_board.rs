pub enum Piece {
    Empty,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Into<u8> for Piece {
    fn into(self) -> u8 {
        match self {
            Piece::Empty    => 0,
            Piece::Pawn     => 1,
            Piece::Knight   => 2,
            Piece::Bishop   => 3,
            Piece::Rook     => 4,
            Piece::Queen    => 5,
            Piece::King     => 6,
        }
    }
}

impl Into<Piece> for u8 {
    fn into(self) -> Piece {
        match self {
            1 => Piece::Pawn,
            2 => Piece::Knight,
            3 => Piece::Bishop,
            4 => Piece::Rook,
            5 => Piece::Queen,
            6 => Piece::King,
            _ => Piece::Empty,
        }
    }
}

pub struct ChessBoard {
    board: [[u8; 8]; 8],
}

impl ChessBoard {
    pub fn new() -> Self {
        Self {
            board: [
                [4, 2, 3, 5, 6, 3, 2, 4],
                [1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [1, 1, 1, 1, 1, 1, 1, 1],
                [4, 2, 3, 5, 6, 3, 2, 4],
            ]
        }
    }

    pub fn get(&self, i: usize, j: usize) -> Piece {
        self.board[i][j].into()
    }

    pub fn set(&mut self, i: usize, j: usize, value: Piece) {
        self.board[i][j] = value.into();
    }
}

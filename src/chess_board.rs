use suoi_rwin::Model;
use suoi_types::{Vector2, Vector3};

use crate::chess_piece::ChessPiece;

pub struct ChessBoard {
    pub model: Model,
    pieces: Vec<ChessPiece>,
}

impl ChessBoard {
    pub fn new(model: Model) -> Self {
        Self {
            model,
            pieces: vec![],
        }
    }

    pub fn add_piece(&mut self, piece: ChessPiece, position: Vector2) {
        let mut piece = piece;
        piece.model.transform.translate(Vector3::new(-5.0 + position.x, 0.0, -5.0 + position.y));
        self.pieces.push(piece);
    }
    
    pub fn pieces(&self) -> &[ChessPiece] {
        &self.pieces
    }
}

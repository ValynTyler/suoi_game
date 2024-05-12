use suoi_rwin::Model;
use suoi_types::{Transform, Vector2, Vector3};

use crate::chess_piece::ChessPiece;

pub struct ChessBoard<'a> {
    pub transform: Transform, 
    pub model: &'a Model,
    pieces: Vec<ChessPiece<'a>>,
}

impl<'a> ChessBoard<'a> {
    pub fn new(model: &'a Model) -> Self {
        Self {
            transform: Transform::default(),
            model,
            pieces: vec![],
        }
    }

    pub fn add_piece(&mut self, mut piece: ChessPiece<'a>, position: Vector2) {
        piece
            .transform
            .translate(Vector3::new(-5.0 + position.x, 0.0, -4.0 + position.y));
        self.pieces.push(piece);
    }

    pub fn pieces(&self) -> &[ChessPiece] {
        &self.pieces
    }

    pub fn start(&mut self, models: &'a Vec<Model>) {
        
    // WHITE
    // pieces
    self.add_piece(ChessPiece::new(&models[4]), Vector2::new(1.0, 1.0));
    self.add_piece(ChessPiece::new(&models[2]), Vector2::new(1.0, 2.0));
    self.add_piece(ChessPiece::new(&models[3]), Vector2::new(1.0, 3.0));
    self.add_piece(ChessPiece::new(&models[5]), Vector2::new(1.0, 4.0));
    self.add_piece(ChessPiece::new(&models[6]), Vector2::new(1.0, 5.0));
    self.add_piece(ChessPiece::new(&models[3]), Vector2::new(1.0, 6.0));
    self.add_piece(ChessPiece::new(&models[2]), Vector2::new(1.0, 7.0));
    self.add_piece(ChessPiece::new(&models[4]), Vector2::new(1.0, 8.0));
    // pawns
    self.add_piece(ChessPiece::new(&models[1]), Vector2::new(2.0, 1.0));
    self.add_piece(ChessPiece::new(&models[1]), Vector2::new(2.0, 2.0));
    self.add_piece(ChessPiece::new(&models[1]), Vector2::new(2.0, 3.0));
    self.add_piece(ChessPiece::new(&models[1]), Vector2::new(2.0, 4.0));
    self.add_piece(ChessPiece::new(&models[1]), Vector2::new(2.0, 5.0));
    self.add_piece(ChessPiece::new(&models[1]), Vector2::new(2.0, 6.0));
    self.add_piece(ChessPiece::new(&models[1]), Vector2::new(2.0, 7.0));
    self.add_piece(ChessPiece::new(&models[1]), Vector2::new(2.0, 8.0));
    // BLACK
    // pieces
    self.add_piece(ChessPiece::new(&models[4]), Vector2::new(8.0, 1.0));
    self.add_piece(ChessPiece::new(&models[2]), Vector2::new(8.0, 2.0));
    self.add_piece(ChessPiece::new(&models[3]), Vector2::new(8.0, 3.0));
    self.add_piece(ChessPiece::new(&models[5]), Vector2::new(8.0, 4.0));
    self.add_piece(ChessPiece::new(&models[6]), Vector2::new(8.0, 5.0));
    self.add_piece(ChessPiece::new(&models[3]), Vector2::new(8.0, 6.0));
    self.add_piece(ChessPiece::new(&models[2]), Vector2::new(8.0, 7.0));
    self.add_piece(ChessPiece::new(&models[4]), Vector2::new(8.0, 8.0));
    // pawns
    self.add_piece(ChessPiece::new(&models[1]), Vector2::new(7.0, 1.0));
    self.add_piece(ChessPiece::new(&models[1]), Vector2::new(7.0, 2.0));
    self.add_piece(ChessPiece::new(&models[1]), Vector2::new(7.0, 3.0));
    self.add_piece(ChessPiece::new(&models[1]), Vector2::new(7.0, 4.0));
    self.add_piece(ChessPiece::new(&models[1]), Vector2::new(7.0, 5.0));
    self.add_piece(ChessPiece::new(&models[1]), Vector2::new(7.0, 6.0));
    self.add_piece(ChessPiece::new(&models[1]), Vector2::new(7.0, 7.0));
    self.add_piece(ChessPiece::new(&models[1]), Vector2::new(7.0, 8.0));
    }
}

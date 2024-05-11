use suoi_rwin::Model;

pub struct ChessPiece {
    pub model: Model,
}

impl ChessPiece {
    pub fn new(model: Model) -> Self {
        Self {
            model,
        }
    }
}

use suoi_rwin::Model;
use suoi_types::Transform;

pub struct ChessPiece<'a> {
    pub transform: Transform,
    pub model: &'a Model,
}

impl<'a> ChessPiece<'a> {
    pub fn new(model: &'a Model) -> Self {
        Self {
            model,
            transform: Default::default(),
        }
    }
}

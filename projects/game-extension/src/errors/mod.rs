pub type Result<T> = std::result::Result<T, GameError>;

#[derive(Debug, Clone)]
pub struct GameError {
    kind: Box<GameErrorKind>,
}

#[derive(Debug, Copy, Clone)]
pub enum GameErrorKind {
    UnknownError,
}

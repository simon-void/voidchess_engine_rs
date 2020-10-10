pub struct ChessError {
    pub msg: String,
    pub kind: ErrorKind,
}

pub enum ErrorKind {
    IllegalPositionFormat,
    IllegalMoveFormat,
    IllegalMove,
}
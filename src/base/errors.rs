#[derive(Debug)]
pub struct ChessError {
    pub msg: String,
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    IllegalConfiguration,
    IllegalFormat,
    IllegalMove,
}
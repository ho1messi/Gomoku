#[derive(Debug)]
pub enum ErrorKind {
    IndexInvalid,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: &'static str,
}

impl Error {
    pub fn create(kind: ErrorKind, message: &'static str) -> Error {
        return Error {
            kind,
            message,
        }
    }
}
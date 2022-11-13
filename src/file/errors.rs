use std::io::Error;

pub enum PayloadError {
    PayloadWriteError(Error),
    FileError(Error)
}
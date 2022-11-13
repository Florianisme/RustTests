use std::io::Error;

pub enum PayloadError {
    PayloadNotFoundError(Error),
    PayloadAlreadyExistsError(Error),
    GeneralFileError(Error)
}

impl PayloadError {
    pub fn get_message(&self) -> String {
        match &*self {
            PayloadError::PayloadNotFoundError(e) => e.to_string(),
            PayloadError::PayloadAlreadyExistsError(e) => e.to_string(),
            PayloadError::GeneralFileError(e) => e.to_string(),
        }
    }
}
use std::io::ErrorKind::{AlreadyExists, NotFound};
use rocket::data::ByteUnit;
use rocket::response::stream::ReaderStream;
use rocket::tokio;
use rocket::tokio::fs::{File, OpenOptions};
use rocket::tokio::io::BufWriter;

use crate::endpoints::payloads::Payload;
use crate::file::errors::PayloadError;
use crate::file::paths;

const FILE_SIZE_LIMIT: ByteUnit = ByteUnit::Gibibyte(1);

pub async fn persist_payload(payload: Payload<'_>) -> Result<(), PayloadError> {
    let open_options = OpenOptions::new().write(true).create_new(true).to_owned();

    let file = open_file(&payload.id, open_options).await?;

    let writer = BufWriter::new(file);

     payload.data.open(FILE_SIZE_LIMIT)
        .stream_to(writer)
        .await.map_err(|e| PayloadError::GeneralFileError(e))?;

    Ok(())
}

pub async fn read_payload(payload_id: &String) -> Result<ReaderStream![File], PayloadError> {
    let open_options = OpenOptions::new().read(true).to_owned();

    let file = open_file(&payload_id, open_options).await?;

    Ok(ReaderStream::one(file))
}

pub async fn delete_payload(payload_id: &String) -> Result<(), PayloadError> {
    delete_file(&payload_id).await
}

async fn open_file(payload_id: &String, open_options: OpenOptions) -> Result<File, PayloadError> {
    let filename = paths::build_filename(&payload_id);
    let result = open_options.open(filename).await;

    match result {
        Ok(file) => Ok(file),
        Err(e) => match e.kind() {
            NotFound => Err(PayloadError::PayloadNotFoundError(e)),
            AlreadyExists => Err(PayloadError::PayloadAlreadyExistsError(e)),
            _ => Err(PayloadError::GeneralFileError(e))
        }
    }
}

async fn delete_file(payload_id: &String) -> Result<(), PayloadError> {
    let filename = paths::build_filename(&payload_id);
    let result = tokio::fs::remove_file(filename).await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            NotFound => Err(PayloadError::PayloadNotFoundError(e)),
            _ => Err(PayloadError::GeneralFileError(e))
        }
    }
}

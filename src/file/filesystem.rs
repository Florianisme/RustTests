use rocket::data::ByteUnit;
use rocket::response::stream::ReaderStream;
use rocket::tokio::fs::File;
use rocket::tokio::io::BufWriter;

use crate::endpoints::payloads::Payload;
use crate::file::errors::PayloadError;
use crate::file::paths;

const FILE_SIZE_LIMIT: ByteUnit = ByteUnit::Gibibyte(1);


pub async fn stream_to_file(payload: Payload<'_>) -> Result<(), PayloadError> {
    let file = create_file(&payload).await.map_err(|e| PayloadError::FileError(e))?;
    let writer = BufWriter::new(file);

     payload.data.open(FILE_SIZE_LIMIT)
        .stream_to(writer)
        .await.map_err(|e| PayloadError::PayloadWriteError(e))?;

    Ok(())
}

pub async fn read_from_file(payload_id: String) -> std::io::Result<ReaderStream![File]> {
    let file = open_file(&payload_id).await?;

    Ok(ReaderStream::one(file))
}

async fn open_file(payload_id: &String) -> Result<File, std::io::Error> {
    let filename = paths::build_filename(&payload_id);
    let file = File::open(filename).await?;

    Ok(file)
}

async fn create_file(payload: &Payload<'_>) -> Result<File, std::io::Error> {
    let filename = paths::build_filename(&payload.id);
    let file = File::create(filename).await?;

    Ok(file)
}

use rocket::tokio::io::BufWriter;
use rocket::tokio::fs::File;
use rocket::data::ByteUnit;
use rocket::response::stream::ReaderStream;

use crate::endpoints::payloads::Payload;
use crate::file::paths;

const FILE_SIZE_LIMIT: ByteUnit = ByteUnit::Gibibyte(1);

pub async fn stream_to_file(payload: Payload<'_>) -> std::io::Result<()> {
    let file = create_file(&payload).await;
    let writer = BufWriter::new(file);

     payload.data.open(FILE_SIZE_LIMIT)
        .stream_to(writer)
        .await?;

    Ok(())
}

pub async fn read_from_file(payload_id: String) -> std::io::Result<ReaderStream![File]> {
    let file = open_file(&payload_id).await;

    Ok(ReaderStream::one(file))
}

async fn open_file(payload_id: &String) -> File {
    let filename = paths::build_filename(&payload_id);
    File::open(filename).await.unwrap()
}

async fn create_file(payload: &Payload<'_>) -> File {
    let filename = paths::build_filename(&payload.id);
    File::create(filename).await.unwrap()
}

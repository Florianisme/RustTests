use rocket::tokio::io::BufWriter;
use rocket::tokio::fs::File;
use rocket::data::ToByteUnit;
use rocket::response::stream::ReaderStream;

use crate::payloads::Payload;

pub async fn stream_to_file(payload: Payload<'_>) -> std::io::Result<()> {
    let file = create_file(&payload).await;
    let writer = BufWriter::new(file);

     payload.data.open(1.gibibytes())
        .stream_to(writer)
        .await?;

    Ok(())
}

pub async fn read_from_file(payload_id: String) -> std::io::Result<ReaderStream![File]> {
    let filename = build_filename(&payload_id);
    let file = File::open(filename).await?;
    Ok(ReaderStream::one(file))
}

async fn create_file(payload: &Payload<'_>) -> File {
    let filename = build_filename(&payload.id);
    File::create(filename).await.unwrap()
}

fn build_filename(payload_id: &String) -> String {
    format!("{}.tmp", payload_id)
}
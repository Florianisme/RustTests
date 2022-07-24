use rocket::tokio::io::BufWriter;
use rocket::tokio::fs::File;
use rocket::data::ToByteUnit;

use crate::payloads::Payload;

pub async fn stream_to_file(payload: Payload<'_>) -> std::io::Result<()> {
    let file = create_file(&payload).await;
    let writer = BufWriter::new(file);

     payload.data.open(1.gibibytes())
        .stream_to(writer)
        .await?;

    Ok(())
}

async fn create_file(payload: &Payload<'_>) -> File {
    let filename = build_filename(&payload);
    File::create(filename).await.unwrap()
}

fn build_filename(payload: &Payload) -> String {
    format!("{}.tmp", payload.id)
}
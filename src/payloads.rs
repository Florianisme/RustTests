
use rocket::Data;
use rocket::response::stream::ReaderStream;
use rocket::tokio::fs::File;
use crate::filesystem;

pub struct Payload<'a> {
    pub(crate) id: String,
    pub(crate) data: Data<'a>
}

#[post("/payload/<payload_id>", data = "<data>")]
pub async fn save_payload(payload_id: String, data: Data<'_>) -> std::io::Result<()> {
    let payload = Payload { id: payload_id, data };

    filesystem::stream_to_file(payload).await
}

#[get("/payload/<payload_id>")]
pub async fn get_payload(payload_id: String) -> std::io::Result<ReaderStream![File]> {
    filesystem::read_from_file(payload_id).await
}
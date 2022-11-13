use rocket::Data;
use rocket::http::Status;
use rocket::response::stream::ReaderStream;
use rocket::tokio::fs::File;

use crate::file::errors::PayloadError;
use crate::file::filesystem;

pub struct Payload<'a> {
    pub(crate) id: String,
    pub(crate) data: Data<'a>
}

#[post("/payload/<payload_id>", data = "<data>")]
pub async fn save_payload(payload_id: String, data: Data<'_>) -> (Status, String) {
    let payload = Payload { id: payload_id.clone(), data };

    let result = filesystem::stream_to_file(payload).await;

    match result {
        Ok(_) => (Status::Created, format!("Payload with id {} persisted", &payload_id)),
        Err(e) => match e {
            PayloadError::PayloadWriteError(_e) => (Status::InternalServerError, format!("Could not write payload with id to file {}", &payload_id)),
            PayloadError::FileError(e) => (Status::InternalServerError, format!("Could not create payload file with id {}: {}", &payload_id, e.to_string()))
        }
    }
}

#[get("/payload/<payload_id>")]
pub async fn get_payload(payload_id: String) -> std::io::Result<ReaderStream![File]> {
    filesystem::read_from_file(payload_id).await
}
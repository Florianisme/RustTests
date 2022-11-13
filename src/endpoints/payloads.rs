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
pub async fn save_payload(payload_id: String, data: Data<'_>) -> (Status, Result<String, String>) {
    let payload = Payload { id: payload_id.clone(), data };

    let result = filesystem::persist_payload(payload).await;

    match result {
        Ok(_) => (Status::Created, Ok(format!("Payload with id {} persisted", &payload_id))),
        Err(e) => match e {
            PayloadError::PayloadAlreadyExistsError(_e) => (Status::Conflict, Err(format!("Payload with id {} already exists", &payload_id))),
            _ => (Status::InternalServerError, Err(format!("Error writing Payload with id {}: {}", &payload_id, e.get_message())))
        }
    }
}

#[get("/payload/<payload_id>")]
pub async fn get_payload(payload_id: String) -> (Status, Result<ReaderStream![File], String>) {
    let result = filesystem::read_payload(&payload_id).await;

    match result {
        Ok(_) => (Status::Ok, Ok(result.ok().unwrap())),
        Err(e) => match e {
            PayloadError::PayloadNotFoundError(_e) => (Status::NotFound, Err(format!("No stored payload found for id {}", &payload_id))),
            _ => (Status::InternalServerError, Err(format!("Error reading Payload with id {}: {}", &payload_id, e.get_message())))
        }
    }
}

#[delete("/payload/<payload_id>")]
pub async fn delete_payload(payload_id: String) -> (Status, Result<String, String>) {
    let result = filesystem::delete_payload(&payload_id).await;

    match result {
        Ok(()) => (Status::Ok, Ok(format!("Payload with id {} deleted", &payload_id))),
        Err(e) => match e {
            PayloadError::PayloadNotFoundError(_e) => (Status::NotFound, Err(format!("No stored payload found for id {}", &payload_id))),
            _ => (Status::InternalServerError, Err(format!("Error deleting Payload with id {}: {}", &payload_id, e.get_message())))
        }
    }
}
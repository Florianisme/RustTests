
use rocket::Data;
use crate::filesystem;

pub struct Payload<'a> {
    pub(crate) id: String,
    pub(crate) data: Data<'a>
}

#[post("/savePayload/<payload_id>", data = "<data>")]
pub async fn save_payload(payload_id: String, data: Data<'_>) -> std::io::Result<()> {
    let payload = Payload { id: payload_id, data };

    filesystem::stream_to_file(payload).await
}
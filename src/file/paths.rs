pub fn build_filename(payload_id: &String) -> String {
    format!("{}.tmp", payload_id)
}
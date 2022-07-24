#[macro_use] extern crate rocket;

mod payloads;
mod filesystem;

#[rocket::main]
async fn main() {
    let _ = rocket::build().mount("/", routes![payloads::save_payload, payloads::get_payload])
        .launch()
        .await
        .expect("Server not started");
}


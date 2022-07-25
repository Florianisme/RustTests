#[macro_use] extern crate rocket;

mod endpoints;
mod file;

#[rocket::main]
async fn main() {
    let _ = rocket::build().mount("/", routes![endpoints::payloads::save_payload, endpoints::payloads::get_payload])
        .launch()
        .await
        .expect("Server not started");
}


#[macro_use]
extern crate rocket;

mod endpoints;
mod file;

#[rocket::main]
async fn main() {
    let routes = routes![endpoints::payloads::save_payload,
        endpoints::payloads::get_payload,
        endpoints::payloads::delete_payload];

    let _ = rocket::build().mount("/", routes)
        .launch()
        .await
        .expect("Server not started");
}


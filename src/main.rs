static SERVER_IP: &'static str = "192.168.0.10";
const SERVER_PORT: u16 = 80;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn root() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("Work in progress...")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Parse gouki.conf here

    // launch server
    HttpServer::new(|| App::new().route("/", web::get().to(root)))
        .bind((SERVER_IP, SERVER_PORT))?
        .run()
        .await
}

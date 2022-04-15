use gouki::conf;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn root() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("Work in progress...")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Parse gouki.conf here
    let conf = conf::get_conf_from_file().unwrap();
    // launch server
    println!("Server start on {}:{}", conf.ip, conf.port);
    HttpServer::new(|| App::new().route("/", web::get().to(root)))
        .bind((conf.ip, conf.port))?
        .run()
        .await
}

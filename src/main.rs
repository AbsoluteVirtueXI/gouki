use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use gouki::conf;

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
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/", "./dist/").index_file("index.html"))
            .route("/test", web::get().to(root)) // doesnt work?
    })
    .bind((conf.ip, conf.port))?
    .run()
    .await
}

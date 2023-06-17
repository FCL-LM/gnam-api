use std::{env, io};
use actix_web::{App, HttpServer, middleware::Logger};
use log::info;


mod gnam;

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Running");

    HttpServer::new(|| {
        let logger = Logger::default();
        
        App::new()
            // enable logger
            .wrap(logger)
            // register HTTP requests handlers
            .service(gnam::index)
            .service(gnam::gnam)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}

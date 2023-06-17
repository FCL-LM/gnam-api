use actix_multipart::form::{tempfile::TempFileConfig, MultipartFormConfig};
use actix_web::{middleware::Logger, App, HttpServer};
use std::io;

use crate::constants::{MAX_FILE_SIZE, MEMORY_LIMIT, TMP_PATH};

mod constants;
mod gnam;

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    std::fs::create_dir_all(TMP_PATH)?;

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(TempFileConfig::default().directory("./data/.tmp"))
            .app_data(
                MultipartFormConfig::default()
                    .total_limit(MAX_FILE_SIZE)
                    .memory_limit(MEMORY_LIMIT),
            )
            // register HTTP requests handlers
            .service(gnam::index)
            .service(gnam::gnam)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}

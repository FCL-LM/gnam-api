use std::fs;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, post, HttpResponse};
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::constants::{TMP_PATH, DATA_PATH};
const APPLICATION_JSON: &str = "application/json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub message: String,
}

// healthcheck endpoint
#[get("/health")]
pub async fn index() -> HttpResponse {
    let response = Response {
        message: String::from("The gnam-api service is up"),
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(response)
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file", limit = "1 GiB")]
    file: TempFile,
}

fn internal_error() -> HttpResponse {
    let response = Response {
        message: String::from("Internal Error"),
    };

    HttpResponse::InternalServerError()
        .content_type(APPLICATION_JSON)
        .json(response)
}

fn status_ok() -> HttpResponse {
    let response = Response {
        message: String::from("Success."),
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(response)
}

// ingestion endpoint
#[post("/gnam")]
pub async fn gnam(MultipartForm(form): MultipartForm<UploadForm>) -> HttpResponse {
    let filename = form.file.file_name.unwrap();
    let path = format!("{}/{}", TMP_PATH, filename.clone());
    info!("ingesting {filename}");

    let err = form.file.file.persist(path.clone());

    if err.is_err() {
        error!("{}", err.err().unwrap());
        return internal_error();
    }

    let data_pathfile = format!("{}/{}", DATA_PATH, filename);
    info!("moving {filename} to data directory...");
    let err = fs::rename(path, data_pathfile);

    if err.is_err() {
        error!("{}", err.err().unwrap());
        return internal_error();
    }

    return status_ok();
}

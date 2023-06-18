use std::{fs, path::Path};

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, post, HttpResponse};
use aws_sdk_s3::primitives::ByteStream;
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::{constants::TMP_PATH, s3mod::get_client};
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
    info!("Ingesting {filename}...");

    let client = get_client().await;

    let err = form.file.file.persist(path.clone());

    if err.is_err() {
        error!("{}", err.err().unwrap());
        return internal_error();
    }

    let body = ByteStream::from_path(Path::new(&path)).await;

    if body.is_err() {
        error!("Creating body: {}", body.err().unwrap());
        return internal_error();
    }

    let multipart_upload_res = client
        .put_object()
        .bucket("sources")
        .key(filename)
        .body(body.unwrap())
        .send()
        .await;

    let _ = fs::remove_file(&path);

    if multipart_upload_res.is_err() {
        error!(
            "Multipart uploading: {}",
            multipart_upload_res.err().unwrap()
        );
        return internal_error();
    }

    return status_ok();
}

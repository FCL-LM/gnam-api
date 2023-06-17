use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, post, HttpResponse};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::format,
    fs::{self, create_dir, remove_file},
    path::Path,
};
pub const APPLICATION_JSON: &str = "application/json";

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
    #[multipart(rename = "file")]
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
    let data_path = String::from("./data");
    let tmp_path = format!("{}/.tmp/", data_path);

    if !Path::new(&tmp_path).exists() {
        let e = create_dir(tmp_path.clone());
        assert!(e.is_ok());
    }

    let f = form.file;
    let filename = f.file_name.unwrap();
    let path = format!("{}{}", tmp_path, filename);
    let tmp_pathfile = f.file.path();

    info!("Saving the file {}", path);

    let err = fs::copy(tmp_pathfile, path.clone());
    let _ = remove_file(tmp_path);

    if err.is_err() {
        error!("{}", err.err().unwrap());
        return internal_error();
    }

    let data_pathfile = format!("{}/{}", data_path, filename);
    let err = fs::rename(path, data_pathfile);

    if err.is_err() {
        error!("{}", err.err().unwrap());
        return internal_error();
    }

    return status_ok();
}

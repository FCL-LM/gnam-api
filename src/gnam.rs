use actix_web::{get, HttpResponse, post};
use serde::{Deserialize, Serialize};
pub const APPLICATION_JSON: &str = "application/json";

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexResponse {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
    pub message: Option<String>,
}

// healthcheck endpoint
#[get("/health")]
pub async fn index() -> HttpResponse {
    let response = IndexResponse {
        message: String::from("The gnam-api service is up"),
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(response)
}

// 
#[post("/gnam")]
pub async fn gnam() -> HttpResponse {
    let response = IndexResponse {
        message: String::from("The gnam-api service is up"),
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(response)
}

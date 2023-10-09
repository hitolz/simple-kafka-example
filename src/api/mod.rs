pub mod test_api;


use actix_web::HttpResponse;
use serde::ser;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct JsonSuccess<T: ser::Serialize> {
    pub code: u32,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct JsonError {
    pub code: u32,
    pub data: Option<String>,
    pub error: Option<String>,
}

pub fn success<T: ser::Serialize>(r: Option<T>) -> HttpResponse {
    HttpResponse::Ok().json(JsonSuccess {
        code: 0,
        data: r,
        error: None,
    })
}

#[allow(dead_code)]
pub fn error(err: Option<String>) -> HttpResponse {
    HttpResponse::Ok().json(JsonError {
        code: 1,
        data: None,
        error: err,
    })
}

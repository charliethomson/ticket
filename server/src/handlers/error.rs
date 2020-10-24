use actix_web::{
    dev::{Body, HttpResponseBuilder, ResponseBody},
    error::ResponseError,
    http::StatusCode,
    HttpResponse,
};
use derive_more::{Display, Error};
//TODO:  use futures::{Stream, StreamExt};
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Debug, Display, Error)]
#[display(fmt = "{} {} {:?}", status_code, status_text, html_path)]
pub struct OffsiteError {
    status_code: u16,
    status_text: String,
    html_path: PathBuf,
}
#[allow(non_snake_case)]
impl OffsiteError {
    pub fn NotFound() -> Self {
        Self {
            status_code: 404,
            status_text: "NotFound".into(),
            html_path: "./static/handlers/404.html".into(),
        }
    }

    pub fn InternalServerError() -> Self {
        Self {
            status_code: 500,
            status_text: "InternalServerError".into(),
            html_path: "./static/handlers/500.html".into(),
        }
    }

    pub fn Unauthorized() -> Self {
        Self {
            status_code: 401,
            status_text: "Unauthorized".into(),
            html_path: "./static/handlers/401.html".into(),
        }
    }

    pub fn body(&self) -> Vec<u8> {
        let mut body = Vec::new();
        // TODO: Unwraps
        let mut file_handle = File::open(&self.html_path).unwrap();
        file_handle.read_to_end(&mut body).unwrap();
        body
    }
}
impl ResponseError for OffsiteError {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status_code).expect("Invalid status code")
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).body(self.body())
    }
}
impl From<OffsiteError> for ResponseBody<Body> {
    fn from(err: OffsiteError) -> ResponseBody<Body> {
        ResponseBody::Other(Body::Bytes(err.body().into()))
    }
}

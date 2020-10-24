mod error;
pub use error::*;

use actix_web::{
    dev::Body,
    http::StatusCode,
    middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers},
};

pub struct OffsiteHandler;
impl OffsiteHandler {
    pub fn new() -> ErrorHandlers<Body> {
        ErrorHandlers::new()
            .handler(StatusCode::NOT_FOUND, |res| {
                let res = res.map_body(|_, _old_body| OffsiteError::NotFound().into()/* TODO: .with(_old_body)*/);
                Ok(ErrorHandlerResponse::Response(res))
            })
            .handler(StatusCode::INTERNAL_SERVER_ERROR, |res| {
                let res =
                    res.map_body(|_, _old_body| OffsiteError::InternalServerError().into()/* TODO: .with(_old_body)*/);
                Ok(ErrorHandlerResponse::Response(res))
            })
            .handler(StatusCode::UNAUTHORIZED, |res| {
                let res = res.map_body(|_, _old_body| OffsiteError::Unauthorized().into()/* TODO: .with(_old_body)*/);
                Ok(ErrorHandlerResponse::Response(res))
            })
    }
}

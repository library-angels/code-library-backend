pub use warp::filters::body::BodyDeserializeError;
pub use warp::reject::{
    not_found, InvalidHeader, InvalidQuery, MethodNotAllowed, MissingCookie, MissingHeader, Reject,
    UnsupportedMediaType,
};

#[derive(Debug)]
pub struct BadRequest(pub String);

impl BadRequest {
    pub fn detail(&self) -> &str {
        &self.0
    }
}

impl Reject for BadRequest {}

#[derive(Debug)]
pub struct Unauthorized(pub String);

impl Unauthorized {
    pub fn detail(&self) -> &str {
        &self.0
    }
}

impl Reject for Unauthorized {}

#[derive(Debug)]
pub struct InternalServerError();

impl Reject for InternalServerError {}

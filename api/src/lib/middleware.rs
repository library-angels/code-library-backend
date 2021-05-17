pub mod rejection {
    use std::convert::Infallible;
    use warp::filters::body::BodyDeserializeError;
    use warp::{reject, reject::MethodNotAllowed, Rejection, Reply};

    use crate::response;

    #[derive(Debug)]
    pub struct NotAuthenticated;

    impl reject::Reject for NotAuthenticated {}

    pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
        let reply = if err.is_not_found() {
            response::not_found("NOT_FOUND")
        } else if let Some(_not_authenticated) = err.find::<NotAuthenticated>() {
            response::unauthorized("AUTHENTICATION_REQUIRED")
        } else if err.find::<BodyDeserializeError>().is_some() {
            response::bad_request("BAD_REQUEST")
        } else if err.find::<MethodNotAllowed>().is_some() {
            response::method_not_allowed()
        } else {
            response::internal_server_error()
        };

        Ok(reply)
    }
}

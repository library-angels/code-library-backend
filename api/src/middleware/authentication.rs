use warp::{Filter, reject, Rejection};
use tarpc::context;

pub fn authentication() -> impl Filter<Extract = (u32,), Error = Rejection> + Copy {
    warp::header::<String>("Authorization").and_then(|authorization_header: String| async move {
        let session_token = match authorization_header.strip_prefix("Bearer ") {
            Some(val) => {
                log::debug!("Found authorization header");
                val
            }
            None => {
                log::debug!("Could not find authorization header");
                return Err(reject::custom(NotAuthenticated));
            }
        };

        let mut client = match crate::rpc::client_connections::identity_client().await {
            Ok(val) => val,
            Err(e) => {
                log::error!("Identity service error: {}", e);
                return Err(reject::custom(NotAuthenticated));
            }
        };

        match client.session_info(context::current(), session_token.to_string()).await {
            Ok(val) => {
                match val {
                    Ok(val) => {
                        Ok(val.sub)
                    },
                    Err(_) => Err(reject::custom(NotAuthenticated)),
                }
            },
            Err(e) => {
                log::debug!("Identity service communication error: {}", e);
                Err(reject::custom(NotAuthenticated))
            },
        }
    })
}

#[derive(Debug)]
pub struct NotAuthenticated;

impl reject::Reject for NotAuthenticated {}

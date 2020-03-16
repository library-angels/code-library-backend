extern crate futures;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use futures::{future, Future, Stream};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use hyper::service::service_fn;

#[derive(Serialize)]
struct BookResponse {
    id: u32,
    attributes: String,
}

#[derive(Deserialize)]
#[serde(tag = "update_book", content = "parameters", rename_all = "lowercase")]
enum BookRequest {
    Attributes {
        id: u32,
        attributes: String
    },
}

fn handle_request(request: BookRequest) -> BookResponse {
    match request {
        BookRequest::Attributes { id, attributes } => {
            BookResponse{ id, attributes }
        }
    }
}

fn service_handler(req: Request<Body>) -> Box<dyn Future<Item = Response<Body>, Error = Error> + Send> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/book") => {
            Box::new(future::ok(Response::new("[WIP] This is /books endpoint".into())))
        },
        (&Method::POST, "/book") => {
            let body = req.into_body().concat2()
                .map(|chunks| {
                    let res = serde_json::from_slice::<BookRequest>(chunks.as_ref())
                        .map(handle_request)
                        .and_then(|response| serde_json::to_string(&response));
                    match res {
                        Ok(body) => {
                           Response::new(body.into()) 
                        },
                        Err(err) => {
                            Response::builder()
                                .status(StatusCode::UNPROCESSABLE_ENTITY)
                                .body(err.to_string().into())
                                .unwrap()
                        },
                    }
                });
            Box::new(body)
        },
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();
            Box::new(future::ok(response))
        }
    }
}

fn main() {
    let local_address = ([127, 0, 0, 1], 8080).into();
    let builder = Server::bind(&local_address);
    let server = builder.serve(|| service_fn(service_handler));
    let server = server.map_err(drop);
    hyper::rt::run(server);
}

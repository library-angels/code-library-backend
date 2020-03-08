use futures::{future, Future};
use hyper::service::service_fn;
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};

fn request_handler(req: Request<Body>) -> impl Future<Item = Response<Body>, Error = Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/book") => {
            future::ok(Response::new("This is /books endpoint".into()))
        },
        (&Method::GET, "/book/authors") => {
            future::ok(Response::new("This is books/authors endpoint...".into()))
        },
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();
            future::ok(response)
        }
    }
}

fn main() {
    let local_address = ([127, 0, 0, 1], 8080).into();
    let builder = Server::bind(&local_address);
    let server = builder.serve(|| service_fn(request_handler));
    let server = server.map_err(drop);
    hyper::rt::run(server);
}

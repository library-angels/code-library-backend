use futures::{future, Future};
use hyper::service::service_fn;
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use std::sync::{Arc, Mutex};
use slab::Slab;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;

lazy_static! {
    //matches: '/', 'index.htm', 'index.html'
    static ref INDEX_PATH: Regex = Regex::new("^/(index\\.html?)?$").unwrap();
    //matches: '/user/', '/user/<id>', '/user/<id>/'
    static ref USER_PATH: Regex = Regex::new("^/user/((?P<user_id>\\d+?)/?)?$").unwrap();
    //matches: '/users', '/users/'
    static ref USERS_PATH: Regex = Regex::new("^/users/?$").unwrap();
    //matches '/books', '/books/'
    static ref BOOKS_PATH: Regex = Regex::new("^/books/?$").unwrap();

}

struct UserData;
type UserId = u64;
type UserDb = Arc<Mutex<Slab<UserData>>>;

fn request_handler(req: Request<Body>, user_db: &UserDb) -> impl Future<Item = Response<Body>, Error = Error> {
    let response = { 
        let method = req.method();
        let path = req.uri().path();
        let mut users = user_db.lock().unwrap();
        
        //check index page using regex
        if INDEX_PATH.is_match(path) {
            if method == Method::GET {
                Response::new("This is /index endpoint".into())
            } else {
                response_builder(StatusCode::METHOD_NOT_ALLOWED)
            }
        }
        // check books using regex
        else if BOOKS_PATH.is_match(path) {
            if method == Method::GET {
                Response::new("This is the /books endpoint".into())
            } else { response_builder(StatusCode::METHOD_NOT_ALLOWED)
            }
        }
        //user list request using regex
         else if USERS_PATH.is_match(path) {
            if method == &Method::GET {
                let list = users.iter()
                    .map(|(id, _)| id.to_string()) 
                    .collect::<Vec<String>>()
                    .join(",");
                Response::new(list.into())
            } else {
                 response_builder(StatusCode::METHOD_NOT_ALLOWED)
            }
        } else if let Some(capture) = USER_PATH.captures(path) {
            let user_id = capture.name("user_id").and_then(|m| {
                m.as_str()
                    .parse::<UserId>()
                    .ok()
                    .map(|x| x as usize)
            });
            match (method, user_id) {
                (&Method::GET, Some(id)) => {
                    if let Some(data) = users.get(id) {
                        Response::new(data.to_string().into())
                    } else { response_builder(StatusCode::NOT_FOUND) }
                },
                (&Method::POST, None) => {
                    let id = users.insert(UserData);
                    Response::new(id.to_string().into())
                },
                (&Method::POST, Some(_)) => {
                    response_builder(StatusCode::BAD_REQUEST)
                },
                (&Method::PUT, Some(id)) => {
                    if let Some(user) = users.get_mut(id) {
                        *user = UserData;
                        response_builder(StatusCode::OK)
                    } else { response_builder(StatusCode::NOT_FOUND) }
                },
                (&Method::DELETE, Some(id)) => {
                    if users.contains(id) {
                        users.remove(id);
                        response_builder(StatusCode::OK)
                    } else { response_builder(StatusCode::NOT_FOUND) }
                },
                _ => { response_builder(StatusCode::METHOD_NOT_ALLOWED) }
            }
        } else {
            response_builder(StatusCode::NOT_FOUND) 
        }
    };
    future::ok(response)
}

fn response_builder(status_code: StatusCode) -> Response<Body> {
Response::builder()
    .status(status_code)
    .body(Body::empty())
    .unwrap()
}

impl fmt::Display for UserData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("{}")
    }
}

fn main() {
let local_address = ([127, 0, 0, 1], 8080).into();
let builder = Server::bind(&local_address);
let user_db = Arc::new(Mutex::new(Slab::new()));
let server = builder.serve(move ||{
    let user_db = user_db.clone();
    service_fn(move |req| request_handler(req, &user_db))
    });
    let server = server.map_err(drop);
    hyper::rt::run(server);
}

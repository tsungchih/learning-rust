extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

use futures::future;
use futures::Stream;
use hyper::{Body, Server, Request, Response, Method, StatusCode};
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::Chunk;

type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn routing(req: Request<Body>) -> BoxFut {
    let mut response: Response<Body> = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("This is the path root /. Hello, world!!!");
        },
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        },
        (&Method::POST, "/echo/uppercase") => {
            let mapping = req.into_body()
                            .map(|chunk: Chunk| {
                                chunk.iter()
                                    .map(|byte| byte.to_ascii_uppercase())
                                    .collect::<Vec<u8>>()
                            });
            *response.body_mut() = Body::wrap_stream(mapping);
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
            *response.body_mut() = Body::from("The page is not found.");
        },
    };
    Box::new(future::ok(response))
}

fn main() {
    pretty_env_logger::init();

    let addr = ([0, 0, 0, 0], 8088).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(routing))
        .map_err(|e| eprintln!("server error {}", e));

    info!("The Server is running now.");
    hyper::rt::run(server);
}

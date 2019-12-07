extern crate hyper;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

use hyper::{Body, Server, Request, Response};
use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;

fn main() {
    pretty_env_logger::init();

    let addr = ([127, 0, 0, 1], 8088).into();

    let server = Server::bind(&addr)
        .serve(|| {
            service_fn_ok(move |_: Request<Body>| {
                Response::new(Body::from("Hello, world!!!"))
            })
        })
        .map_err(|e| eprintln!("server error {}", e));

    rt::run(server);
    info!("The Server is running now.")
}

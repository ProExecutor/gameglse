use std::fs::File;
use std::io::prelude::*;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use tokio::runtime::Builder;

async fn serve_html(_: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut file = File::open("index.html").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    Ok(Response::new(Body::from(contents)))
}

#[tokio::main]
async fn main() {
    let make_svc = make_service_fn(|_conn| {
        async {
            Ok::<_, hyper::Error>(service_fn(serve_html))
        }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

use hyper::server::{Server, Request, Response};
use hyper::uri::RequestUri;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use super::*;

pub fn serve() {
    Server::http("127.0.0.1:8000").unwrap().handle(static_file_handler).unwrap();
}

fn static_file_handler(req: Request, mut res: Response) {
    let req_path = match req.uri {
        RequestUri::AbsolutePath(p) => p,
        _ => {
            *res.status_mut() = hyper::status::StatusCode::BadRequest;
            let body = b"<h1>400: Bad request</h1>";
            return res.send(body).unwrap();
        }
    };

    let path = PathBuf::from(&*BUILD_PATH).join(&req_path[1..]);

    let serve_path = if path.is_file() {
        path
    } else {
        path.join("index.html")
    };

    if serve_path.exists() {
        let mut file = File::open(serve_path).unwrap();
        let mut buffer: Vec<u8> = vec![];

        file.read_to_end(&mut buffer).unwrap();

        res.send(&buffer).unwrap();
    } else {
        let body = b"<h1>404: Page not found</h1>";

        *res.status_mut() = hyper::status::StatusCode::NotFound;
        res.send(body).unwrap();
    }
}

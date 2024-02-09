use super::router::Router;
use http::http_request::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;


pub struct Server<'a> {
    server_addr: &'a str,
}

impl<'a> Default for Server<'a> {
    fn default() -> Self {
        Server {
            server_addr: "127.0.0.1:8000",
        }
    }
}
impl<'a> Server<'a> {
    pub fn new(addr: &'a str) -> Self {
        Server {
            server_addr: addr,
        }
    }

    pub fn run(&self) {
        let server_listener = TcpListener::bind(self.server_addr).unwrap();
        println!(" Running on {}", self.server_addr);

        for stream in server_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established!");

            let mut read_buffer = [0; 1024];
            stream.read(&mut read_buffer).unwrap();
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}
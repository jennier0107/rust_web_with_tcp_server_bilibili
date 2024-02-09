use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use std::io::prelude::*;
use http::{http_request::HttpRequest, http_response::HttpResponse};
use http::http_request::{Method, Resource};


pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            Method::Get => {
                match &req.resource {
                    Resource(r)  => {
                        let route_ : Vec<&str> = r.split("/").collect();
                        match route_[1] {
                            "api" => {
                                let resp: HttpResponse = WebServiceHandler::handle(&req);
                                let _ = resp.send_response(stream);
                            },
                            _ => {
                                let resp: HttpResponse = StaticPageHandler::handle(&req);
                                let _ = resp.send_response(stream);
                            }
                        }
                    }
                }
            }
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
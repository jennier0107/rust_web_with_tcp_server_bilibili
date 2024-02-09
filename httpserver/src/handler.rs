use http::{http_request::{HttpRequest, Resource}, http_response::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;


pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;

    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);

        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}

pub struct PageNotFoundHandler;
pub struct StaticPageHandler;
pub struct WebServiceHandler;



#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String
}


impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let Resource(r) = &req.resource;
        let route_: Vec<&str> = r.split("/").collect();
        match route_[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut m: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css") {
                        m.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        m.insert("Content-Type", "text/javascript");
                    } else {
                        m.insert("Content-Type","text/html");
                    }
                    HttpResponse::new("200", Some(m), Some(contents))
                }
                None => HttpResponse::new("404", None, Self::load_file("404.html"))
            }
        }
    }
}

impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_contents = fs::read_to_string(full_path);
        let orders: Vec<OrderStatus> = serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
        orders
    }
}

impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let Resource(r) = &req.resource;
        let route_: Vec<&str> = r.split("/").collect();

        match route_[2] {
            "shipping" if route_.len() > 2 && route_[3] == "orders" => {
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");

                HttpResponse::new("200", Some(headers), body)
            },
            _ => HttpResponse::new("404", None, Self::load_file("404.html"))
        }
    }
}
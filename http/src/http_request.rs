use std::collections::HashMap;
use crate::common::HttpVersion;

/// http 请求方法
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

/// http 请求路径
#[derive(Debug, PartialEq)]
pub struct Resource(pub String);

/// http 请求结构体
#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: HttpVersion,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = HttpVersion::Uninitialized;
        let mut parsed_resource = Resource("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";


        for line in req.lines() {
            // 请求行
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {} else {
                parsed_msg_body = line;
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

/// 解析请求行
fn process_req_line(s: &str) -> (Method, Resource, HttpVersion) {
    let mut words = s.split_whitespace();

    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource(resource.into()),
        version.into()
    )
}

/// 解析 header 行
fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    if let Some(v) = header_items.next() {
        value = v.to_string();
    }

    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into_get() {
        let get_: Method = "GET".into();
        assert_eq!(get_, Method::Get);
    }

    #[test]
    fn test_method_into_other() {
        let other_: Method = "other".into();
        assert_eq!(other_, Method::Uninitialized);
    }

    #[test]
    fn test_version_into() {
        let v1: HttpVersion = "HTTP/1.1".into();
        assert_eq!(v1, HttpVersion::V1_1)
    }

    #[test]
    fn test_read_http() {
        let raw_http_string: String = String::from("GET / HTTP/1.1\r\nHost: developer.mozilla.org\r\nAccept-Language: fr\r\n\r\n");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " developer.mozilla.org".into());
        headers_expected.insert("Accept-Language".into(), " fr".into());

        let req: HttpRequest = raw_http_string.into();

        assert_eq!(Method::Get, req.method);
        assert_eq!(HttpVersion::V1_1, req.version);
        assert_eq!(Resource("/".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}
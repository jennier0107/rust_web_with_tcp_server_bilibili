use std::collections::HashMap;
use std::io::{Result, Write};

use
crate::common::HttpVersion;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a HttpVersion,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: &HttpVersion::V1_1,
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(value: HttpResponse<'a>) -> Self {
        let res = value.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            res.version(),
            res.status_code(),
            res.status_text(),
            res.headers(),
            res.body().len(),
            res.body()
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response = Self::default();

        if status_code != "200" {
            response.status_code = status_code.into();
        }

        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };

        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Unknown Error".into()
        };

        response.body = body;

        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let response_string: String = String::from(self.clone());
        let _ = write!(write_stream, "{}", response_string);

        Ok(())
    }

    fn version(&self) -> &HttpVersion {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let header_map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        for (k, v) in header_map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v)
        }
        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            None => "",
            Some(body) => body.as_str(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new(
            "200",
            None,
            Some("xxxx".into())
        );
        let response_expected = HttpResponse {
            version: &HttpVersion::V1_1,
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into())
        };

        assert_eq!(response_actual, response_expected);
    }
}
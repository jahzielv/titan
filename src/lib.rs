use http::Uri;
use std::fs;
use std::path::Path;
use toml::Value;

pub enum StatusCode {
    Code20,
    Code51,
}

impl StatusCode {
    pub fn code(&self) -> u8 {
        match *self {
            StatusCode::Code20 => 20u8,
            StatusCode::Code51 => 51u8,
        }
    }
}

pub struct Response {
    pub status: StatusCode,
    pub body: Option<String>,
    pub meta: Option<String>,
}

impl Response {
    pub fn not_found() -> Response {
        Response {
            status: StatusCode::Code51,
            body: None,
            meta: None,
        }
    }
    pub fn new(code: StatusCode) -> Response {
        Response {
            status: code,
            body: None,
            meta: None,
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let meta = match self.meta {
            Some(v) => v,
            None => String::new(),
        };
        let body = match self.body {
            Some(v) => v,
            None => String::new(),
        };
        Vec::from(format!("{} {}\r\n{}", self.status.code(), meta, body).as_bytes())
    }

    pub fn set_body(&mut self, body: String) {
        self.body = Option::from(body)
    }

    pub fn set_meta(&mut self, meta: String) {
        self.meta = Option::from(meta)
    }
}

pub fn build_response(body: &[u8]) -> Vec<u8> {
    [format!("{} {}\r\n", 20, "text/gemini").as_bytes(), body].concat()
}

pub fn parse_uri(uri: &str) -> String {
    String::from(uri.parse::<Uri>().unwrap().path())
}

pub fn find_route(route: &str) -> String {
    println!("looking for {:?}", route);
    let config_string = fs::read_to_string("Titan.toml").unwrap();
    let map = config_string.parse::<Value>().unwrap();
    if let Value::Table(t) = &map["routes"] {
        match t.get(route) {
            Some(v) => {
                return match v.as_str() {
                    Some(s) => String::from(s),
                    None => String::from("not a string"),
                }
            }
            None => return String::from("error"),
        };
    } else {
        String::from("error")
    }
}

pub fn request_to_uri(data: &mut [u8]) -> String {
    let mut req_asvec: Vec<u8> = Vec::new();
    for b in data.iter() {
        if *b as char == '\r' {
            break;
        }
        req_asvec.push(b.clone());
    }
    String::from_utf8_lossy(&req_asvec).to_string()
}

pub fn get_body(file_path: &str) -> String {
    let file_to_serve = find_route(file_path);
    fs::read_to_string(Path::new(&file_to_serve)).unwrap()
}

// fn get_route(path: &str, table: Map) -> String {
//     // if the current key is a string that matches path, return the value
//     // if the current key is a table, recurse
// }

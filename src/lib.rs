use http::Uri;
use std::fs;
use std::path::Path;
use toml::Value;

pub enum StatusCode {
    Input,
    SensInput,
    Empty,
    Success,
    RedirTemp,
    RedirPerm,
    TempFail,
    ServerUnavail,
    CGIErr,
    ProxyErr,
    SlowDown,
    PermFail,
    NotFound,
    Gone,
    ProxReqRef,
    BadReq,
    ClientCert,
    CertNotAuth,
    InvalidCert,
}

impl StatusCode {
    pub fn code(&self) -> u8 {
        match *self {
            StatusCode::Empty => 0u8,
            StatusCode::Success => 20u8,
            StatusCode::NotFound => 51u8,
            StatusCode::Input => 10u8,
            StatusCode::SensInput => 11u8,
            StatusCode::RedirTemp => 30u8,
            StatusCode::RedirPerm => 31u8,
            StatusCode::TempFail => 40u8,
            StatusCode::ServerUnavail => 41u8,
            StatusCode::CGIErr => 42u8,
            StatusCode::ProxyErr => 43u8,
            StatusCode::SlowDown => 43u8,
            StatusCode::PermFail => 50u8,
            StatusCode::Gone => 52u8,
            StatusCode::ProxReqRef => 53u8,
            StatusCode::BadReq => 59u8,
            StatusCode::ClientCert => 60u8,
            StatusCode::CertNotAuth => 61u8,
            StatusCode::InvalidCert => 63u8,
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
            status: StatusCode::NotFound,
            body: None,
            meta: Option::from("File not found".to_owned()),
        }
    }
    pub fn new() -> Response {
        Response {
            status: StatusCode::Empty,
            body: None,
            meta: None,
        }
    }
    pub fn with_status(code: StatusCode) -> Response {
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

    pub fn set_body(&mut self, body: &str) {
        self.body = Option::from(body.to_owned())
    }

    pub fn set_meta(&mut self, meta: &str) {
        self.meta = Option::from(meta.to_owned())
    }

    pub fn set_status(&mut self, status: StatusCode) {
        self.status = status
    }
}

pub fn build_response(body: &[u8]) -> Vec<u8> {
    [format!("{} {}\r\n", 20, "text/gemini").as_bytes(), body].concat()
}

pub fn parse_uri(uri: &str) -> String {
    String::from(uri.parse::<Uri>().unwrap().path())
}

pub fn find_route(route: &str) -> Option<String> {
    println!("looking for {:?}", route);
    let config_string = fs::read_to_string("Titan.toml").unwrap();
    let map = config_string.parse::<Value>().unwrap();
    if let Value::Table(t) = &map["routes"] {
        match t.get(route) {
            Some(v) => {
                return match v.as_str() {
                    Some(s) => Option::from(String::from(s)),
                    None => None,
                }
            }
            None => return None,
        };
    } else {
        None
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

pub fn get_body(file_path: &str) -> Option<String> {
    let file_to_serve = find_route(file_path);
    match file_to_serve {
        Some(file) => Option::from(fs::read_to_string(Path::new(&file)).unwrap()),
        None => None,
    }
}

// fn get_route(path: &str, table: Map) -> String {
//     // if the current key is a string that matches path, return the value
//     // if the current key is a table, recurse
// }

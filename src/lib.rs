use http::Uri;
use serde_derive::Deserialize;
use std::fs;
use toml::map::Map;
use toml::Value;
// #[derive(Deserialize, Debug)]
// struct Config {
//     routes: ,
// }

// #[derive(Deserialize, Debug)]
// struct Route {

// }

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

// fn get_route(path: &str, table: Map) -> String {
//     // if the current key is a string that matches path, return the value
//     // if the current key is a table, recurse
// }

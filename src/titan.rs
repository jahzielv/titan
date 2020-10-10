use crossbeam::scope;
use failure::{Error, Fail};
use libtitan::StatusCode;
use libtitan::{parse_uri, request_to_uri, Response};
use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use toml::Value;
// use serde_derive::Deserialize;

// #[derive(Deserialize)]
// struct Config {
//     server_config: ServerConfig,
//     routes: Routes
// }

// #[derive(Deserialize)]
// struct ServerConfig {
//     domain: String,
//     pfx: String
// }

#[derive(Fail, Debug)]
pub enum PfxError {
    #[fail(display = "Pfx key not defined in Titan.toml!")]
    PfxKeyNotFound,
    #[fail(display = "This is a fake error")]
    Fake,
}

type Handler = Box<dyn Fn(&mut Response) + Send + Sync>;

pub struct Titan {
    handlers: HashMap<String, Handler>,
    pfx_file: String,
    // pfx_password: String,
}

impl Titan {
    pub fn new() -> Titan {
        Titan {
            handlers: HashMap::new(),
            pfx_file: String::new(),
            // pfx_password: String::new(),
        }
    }

    pub fn get(&mut self, route: &str, handler: Handler) {
        match self.handlers.insert(route.to_owned(), handler) {
            Some(_) => {}
            None => {}
        }
    }

    // error possibilities:
    // 1. pfx key doesn't exist in file
    // 2. pfx key not set to a string
    // 3. pfx key is a string but not a valid path or file doesn't exist
    pub fn read_config_file(&mut self) -> Result<(), PfxError> {
        let config_string = fs::read_to_string("Titan.toml").unwrap();
        let map = config_string.parse::<Value>().unwrap();
        if let Value::Table(t) = &map["server_config"] {
            let config_table = t.clone();
            // println!("{}", config_table["pfx"]);
            // let pfx_val_res = std::panic::catch_unwind(|| &config_table["pfx"])?;
            let pfx_val = &config_table["pfx"];

            match pfx_val.as_str() {
                Some(pfx_file) => self.set_pfx_file(pfx_file),
                None => eprintln!("pfx file not set in Titan.toml"),
            };
        } else {
            return Err(PfxError::Fake);
        };

        // set up routes

        if let Value::Table(t) = &map["routes"] {
            let routes_table = t.clone();
            for (key, value) in routes_table {
                self.get(
                    &key,
                    Box::new(move |res| {
                        let file_to_serve = value.as_str().unwrap();
                        let file_contents = &fs::read_to_string(file_to_serve).unwrap();
                        res.set_body(file_contents)
                            .set_meta("text/gemini")
                            .set_status(StatusCode::Success);
                    }),
                );
            }
        };
        Ok(())
    }

    pub fn set_pfx_file(&mut self, file_path: &str) {
        self.pfx_file = file_path.to_owned();
    }
}

fn global_handler(t: &Titan, stream: &mut TlsStream<TcpStream>) {
    let mut data = [0 as u8; 1000]; // using 50 byte buffer
    stream.read(&mut data).unwrap();
    let route = &parse_uri(&request_to_uri(&mut data));
    println!("route requested: {:?}", route);
    let mut res = Response::new();
    match t.handlers.get(route) {
        Some(h) => {
            h(&mut res);
            stream.write(&res.to_bytes()).unwrap();
            stream.shutdown().unwrap()
        }
        None => {
            let not_found = Response::not_found();
            stream.write(&not_found.to_bytes()).unwrap();
            stream.shutdown().unwrap()
        }
    }
}

/// Use the pfx file to create an identity for TLS connections.
fn server_auth(pfx_file: &str) -> Result<Identity, Error> {
    println!("saved pfx file: {}", pfx_file);
    let mut file = File::open(pfx_file)?;
    let mut identity = vec![];
    file.read_to_end(&mut identity)?;
    let identity = Identity::from_pkcs12(&identity, &std::env::var("TITAN_CERT_KEY")?)?;
    Ok(identity)
}

pub fn start(t: &Titan) -> Result<(), Error> {
    let identity = server_auth(&t.pfx_file)?;
    let acceptor = TlsAcceptor::new(identity)?;
    let acceptor = Arc::new(acceptor);

    let listener = TcpListener::bind("0.0.0.0:1965").unwrap();
    println!("Server listening on port 1965");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                scope(|scope| {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    let acceptor = acceptor.clone();
                    scope.spawn(move |_| {
                        let mut stream = acceptor.accept(stream).unwrap();

                        global_handler(t, &mut stream);
                    });
                })
                .unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
    Ok(())
}

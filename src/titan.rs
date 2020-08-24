// use crossbeam_utils::thread;
use crossbeam::scope;
use libtitan::{get_body, parse_uri, request_to_uri, Response, StatusCode};
use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
// use std::thread;

type Handler = Box<dyn Fn(&mut Response) + Send + Sync>;

pub struct Titan {
    handlers: HashMap<String, Handler>,
}

impl Titan {
    pub fn new() -> Titan {
        Titan {
            handlers: HashMap::new(),
        }
    }

    pub fn get(&mut self, route: &str, handler: Handler) {
        match self.handlers.insert(route.to_owned(), handler) {
            Some(_) => {}
            None => eprintln!("ERROR in get"),
        }
    }

    pub fn global_handler(self, stream: &mut TlsStream<TcpStream>) {
        let mut data = [0 as u8; 1000]; // using 50 byte buffer
        stream.read(&mut data).unwrap();
        let route = &parse_uri(&request_to_uri(&mut data));
        println!("route requested: {:?}", route);
        let mut res = Response::new();
        match self.handlers.get(route) {
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
}

pub fn start(t: &Titan) {
    let mut file = File::open("certificate.pfx").unwrap();
    let mut identity = vec![];
    file.read_to_end(&mut identity).unwrap();
    let identity = Identity::from_pkcs12(&identity, "password").unwrap();
    let acceptor = TlsAcceptor::new(identity).unwrap();
    let acceptor = Arc::new(acceptor);
    // let server = Arc::new(t);

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
}

pub fn global_handler(t: &Titan, stream: &mut TlsStream<TcpStream>) {
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

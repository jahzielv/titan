use libtitan::{get_body, parse_uri, request_to_uri, Response, StatusCode};
use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

type Handler = Box<dyn FnOnce(Response)>;

pub struct Titan {
    handlers: HashMap<String, Handler>,
}

impl Titan {
    pub fn get<F>(&mut self, route: &str, handler: Handler) {
        match self.handlers.insert(route.to_owned(), handler) {
            Some(v) => {}
            None => eprintln!("ERROR in get"),
        }
    }
    pub fn basic_handler(stream: &mut TlsStream<TcpStream>) {
        let mut data = [0 as u8; 1000]; // using 50 byte buffer
        stream.read(&mut data).unwrap();
        let path = &parse_uri(&request_to_uri(&mut data));
        println!("raw path {:?}", path);
        let mut response: Response; //Response::new(StatusCode::Code20);
                                    // let body = get_body(path);
        match get_body(path) {
            Some(body) => {
                response = Response::with_status(StatusCode::Code20);
                response.set_body(body);
                response.set_meta("text/gemini".to_owned());
                stream.write(&response.to_bytes()).unwrap();
            }
            None => {
                response = Response::not_found();
                stream.write(&response.to_bytes()).unwrap();
                /*
                let app = Titan::new();
                app.get("/foo/bar", |req, res| {res::send("Some text")});
                app.start();
                */
            }
        }

        // let no = Response::not_found();
        // stream.write(&no.to_bytes()).unwrap();
        stream.shutdown().unwrap();
    }
    pub fn start() {
        let mut file = File::open("certificate.pfx").unwrap();
        let mut identity = vec![];
        file.read_to_end(&mut identity).unwrap();
        let identity = Identity::from_pkcs12(&identity, "password").unwrap();
        let acceptor = TlsAcceptor::new(identity).unwrap();
        let acceptor = Arc::new(acceptor);

        let listener = TcpListener::bind("0.0.0.0:1965").unwrap();
        println!("Server listening on port 1965");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    let acceptor = acceptor.clone();
                    thread::spawn(move || {
                        let mut stream = acceptor.accept(stream).unwrap();
                        Titan::basic_handler(&mut stream);
                    });
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
}

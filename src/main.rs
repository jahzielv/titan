use libtitan::{find_route, parse_uri, request_to_uri, Response, StatusCode};
use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::Arc;
use std::thread;

fn handle_client(mut stream: TlsStream<TcpStream>) {
    let mut data = [0 as u8; 1000]; // using 50 byte buffer
    stream.read(&mut data).unwrap();
    let path = &parse_uri(&request_to_uri(&mut data));
    println!("raw path {:?}", path);

    let file_to_serve = find_route(path);
    let mut response = Response::new(StatusCode::Code20);
    let body = fs::read_to_string(Path::new(&file_to_serve)).unwrap();
    response.set_body(body);
    response.set_meta("text/gemini".to_owned());
    println!("file requested: {:?}", file_to_serve);
    stream.write(&response.to_bytes()).unwrap();
    stream.shutdown().unwrap();
}

fn main() {
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
                    let stream = acceptor.accept(stream).unwrap();
                    handle_client(stream);
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

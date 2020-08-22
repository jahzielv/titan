use libtitan::{build_response, find_route, parse_uri};
use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::fs::metadata;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::path::Path;
use std::sync::Arc;
use std::thread;

fn handle_client(mut stream: TlsStream<TcpStream>) {
    let mut data = [0 as u8; 1000]; // using 50 byte buffer
    let mut req_asvec: Vec<u8> = Vec::new();
    stream.read(&mut data).unwrap();
    for b in data.iter() {
        if *b as char == '\r' {
            break;
        }
        req_asvec.push(b.clone());
    }
    println!("{:?}", String::from_utf8_lossy(&req_asvec));
    let path = &parse_uri(&String::from_utf8_lossy(&req_asvec));
    println!("raw path {:?}", path);
    // let path = match path == "/" {
    //     true => "root",
    //     false => path,
    // };
    let file_to_serve = find_route(path);
    println!("file requested: {:?}", file_to_serve);
    let mut input = File::open(Path::new(&file_to_serve)).unwrap();
    // let buffered = BufReader::new(input);
    let metadata = metadata(&file_to_serve).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    input.read(&mut buffer).expect("buffer overflow");

    stream.write(&build_response(&buffer)).unwrap();
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

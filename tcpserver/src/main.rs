use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let l = TcpListener::bind("127.0.0.1:9999");

    let listener = match l {
        Ok(l) => l,
        Err(e) => panic!("bind socket error: {:?}", e),
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer[..]);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

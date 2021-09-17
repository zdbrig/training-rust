use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 8192];

    let number: usize = stream.read(&mut buffer).unwrap();

    println!("{} bytes value: {}", 
            number , 
           buffer[0]);

    let response = "🍎 Narjassi \r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
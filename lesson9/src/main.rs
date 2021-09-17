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
    let mut s:[u8;8] = [0; 8];
    s.copy_from_slice(&buffer[0..8]);
    let num = u64::from_be_bytes(s);
    s.copy_from_slice(&buffer[8..16]);
    let num2 = u64::from_be_bytes(s);
    println!("{} bytes value: {} {}", 
            number , 
           num,
           num2
        );

    let response = "🍎 Narjassi \r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
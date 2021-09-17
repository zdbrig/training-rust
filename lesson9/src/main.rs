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

     stream.read(&mut buffer).unwrap();
    let length = (buffer[0] as usize) * 16;
    println!(" reading {} numbers" , length);
    let mut i = 0 as usize;
    while i < length {
        let mut s:[u8;8] = [0; 8];
    s.copy_from_slice(&buffer[(i+1)..(i+9)]);
    let num = u64::from_be_bytes(s);
    s.copy_from_slice(&buffer[(i+9)..(i+17)]);
    let num2 = u64::from_be_bytes(s);
    println!(" value: {} , {}",    
           num,
           num2
        );
        i = i+16;
    }
    

    let response = "🍎 Narjassi \r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
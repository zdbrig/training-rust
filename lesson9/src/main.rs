use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use rand::Rng;

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
    
    let mut buffer = [0; 8192];

    

    let mut rng = rand::thread_rng();
    let number: u8 = rng.gen::<u8>() % 10 + 1;
    buffer[0] = number;
    println!("writing {}",number);

    let mut i = 0usize;

    while i < (number as usize *16 + 1) {
        let num1 : u64 = rng.gen();
        let num2 : u64 = rng.gen();
        let bytes1 = num1.to_be_bytes();
        let bytes2 = num2.to_be_bytes();
        buffer[(i+1)..(i+9)].clone_from_slice(&bytes1);
        buffer[(i+9)..(i+17)].clone_from_slice(&bytes2);
        i = i+16;

        println!(" value: {} , {}",    
        num1,
        num2
     );
    }
   
    stream.write(&buffer[0..((number as usize) *64+1)]).unwrap();
    stream.flush().unwrap();
}

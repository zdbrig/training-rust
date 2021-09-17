use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use rand::Rng;
use std::str;



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
    let mut s1:[u8;4] = [0; 4];
    let mut s:[u8;8] = [0; 8];
    let mut s2:[u8;22] = [0; 22];
    s1.copy_from_slice(&buffer[(i+1)..(i+5)]);
    let num = u32::from_be_bytes(s1);
    s.copy_from_slice(&buffer[(i+5)..(i+13)]);
    let num2 = u64::from_be_bytes(s);
    s2.copy_from_slice(&buffer[(i+13)..(i+35)]);
    
    //s.copy_from_slice(&buffer[(i+36)..(i+37)]);
    let bool1 = buffer[i+35];
    println!(" value: {} , {} , {:?} , {}",    
           num,
           num2,
           str::from_utf8(&s2),
           bool1==1
        );
        i = i+36;
    }
    
   
    let response = "🍎 Narjassi \r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    ////////
    

    let mut rng = rand::thread_rng();
    let number: u8 = rng.gen::<u8>() % 10 + 1;
    buffer[0] = number;
    println!("writing {}",number);

    let mut i = 0usize;

    while i < (number as usize *16 + 1) {
        let num1 : u32 = rng.gen();
        let num2 : u64 = rng.gen();
       
        let bool1:u8=1;
        let bytes1 = num1.to_be_bytes();
        let bytes2 = num2.to_be_bytes();
        let str1 = b"i m amal";
    
        //let bytes4=bool1.to_be_bytes();
        
        buffer[(i+1)..(i+5)].clone_from_slice(&bytes1);
        buffer[(i+5)..(i+13)].clone_from_slice(&bytes2);
       buffer[(i+13)..(i+21)].clone_from_slice(str1);
        buffer[(i+21)];
        i = i+22;

        println!(" jawaher: {} , {},{} , {:?}",    
        num1,
        num2,
        bool1,
        str1
     );
    }
   
    stream.write(&buffer[0..((number as usize) *64+1)]).unwrap();
    stream.flush().unwrap();


}

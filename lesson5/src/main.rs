use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    // --snip--
    println!("------- Reading a file ");
    let filename = "/Users/karima/trainingrust/lesson5/poem.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let contents1 = fs::read(filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
    println!("len {}", contents1.len());
    for i in contents1.iter() {
        println!("{}", i);
    }
    
    let mut file2 = File::open("/Users/karima/trainingrust/lesson5/poem.txt").unwrap();
    let mut buffer = Vec::new();
    let f= file2.read_to_end(&mut buffer).unwrap();
    println!("Last file :\n{}", f);
    for i in buffer.iter(){
        println!("buffer {}", i);
    }
    println!("---------read exact -----");
    let mut file3 = File::open("/Users/karima/trainingrust/lesson5/poem.txt").unwrap();
    let mut buffer = [0; 20];
    let f=file3.read_exact(&mut buffer);
    println!("read exact {:?}", f);
}

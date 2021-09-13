
use crate::karima;


pub fn say_hello() {
    println!("Hello from Jawaher");
    println!("Jawaher is calling Karima");
    karima::say_hello();
    println!("Jawaher is calling Halima");
    sqoinlib::halima::say_hello();
}
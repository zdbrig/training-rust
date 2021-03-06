mod sqoin_module {
    pub mod sqoin_sub_module {
        pub struct Sqoiner {
            pub number: i32,
        }

        pub trait SqoinRules {
            fn shownumber(sqoiner: &Sqoiner);
        }

        pub fn print_sqoiner(sqoiner: &Sqoiner) -> () {
            println!(" Printing Sqoin {}", sqoiner.number);
        }
    }
}

mod karima {

    pub fn say_hello() {
        println!(" I am Karima");
    }
}

mod jawaher;
mod khouloud;
use std::fmt::{Display, Formatter, Result};
fn main() {
    println!("------- Modules ----------");

    sqoin_module::sqoin_sub_module::print_sqoiner(&sqoin_module::sqoin_sub_module::Sqoiner {
        number: 10,
    });
    karima::say_hello();
    khouloud::say_hello();
    jawaher::jawaherimpl::say_hello();

    println!("------- Variable----------");

    let x = 100u32 + (10u8 as u32);

    let y: u32 = 1000u32;

    let f: f32 = 0.001f32;

    let b: bool = {
        let mut ret = false;
        if x < y {
            ret = true;
        }
        ret
    };

    let func = move |z: u32| {
        let mut s = x;
        s = s * z * z;
        s
    };
    println!(" value of x is {}", x);
    println!(" value of y is {}", y);
    println!(" value of f is {}", f);
    println!(" value of b is {}", b);
    println!(" value of func(y) is {}", func(y));

    println!("--------- Tables --------");

    let mut table: [u32; 10] = [10; 10];

    for i in 0..10 {
        print!(" {}", table[i]);
    }
    println!();
    for i in 0..10 {
        table[i] = (i + 1) as u32;
    }
    for i in 0..10 {
        print!(" {}", table[i]);
    }

    println!("--------- Tuples --------");

    let tup: (u32, [u8; 4]) = {
        println!("assigning tuple");
        (4u32, [1u8, 2, 3, 4])
    };
    println!("{}", tup.0);

    let z = (10,);

    println!("{}", z.0);

    println!("--------- Vecteur --------");

    let mut vec1: Vec<u32> = Vec::new();
    vec1.push(y);
    vec1.push(y);

    for i in vec1 {
        print!(" {} ", i);
    }

    println!();

    println!("--------- Memory Management Variable --------");

    let m1: u32 = 8u32;

    let m2 = m1;

    println!(" {} {} ", m1, m2);

    let mut b1 = Box::new(10u32);

    let b2 = b1;
    b1 = Box::new(11u32);

    print_box(b1);
    b1 = print_box(b2);
    print_box(b1);

    println!("--------- Memory Management Reference --------");

    let mut v3 = vec![10u32];
    {
        let mut refv3 = &mut v3;
        refv3.push(11u32);
    }
    println!("{} ", &v3[1]);

    println!("--------- Memory Management Slices --------");

    let table = [1, 2, 3, 4, 5, 6];

    let slice = &table[1..3];

    print_slice(slice);
    print_slice(&v3);

    println!("--------- String --------");

    let s = r"b\acem";
    let s1 = "b\\acem".to_string();
    let mut s2 = String::new();
    s2 += "bacem";

    let a = s2.clone();
    let b = s2.clone();
    print!("{} {} {} ", s, s1, s2);
    println!("--------- Enum and patterns  --------");

    let trainer: Trainee = Trainee::Bacem(9u32);
    let trainer1: Trainee = Trainee::Amal("Amal2".to_string());
    let trainer2: Trainee = Trainee::Jawaher(true);

    let t1 = &trainer1;

    use Trainee::*;
    let a = match t1 {
        Bacem(9) => 9u32,
        Bacem(x) => *x,
        Amal(x) => x.len() as u32,
        _ => {
            println!(" I dont know");
            10u32
        }
    };

    println!(" a= {}", a);

    println!("--------- Functions --------");

    println!(" sum of integers : {}", sum::<u32>(10u32, 20u32));
    println!(" sum of integers : {}", sum(1.2f64, 1.3f64));

    //   println!("{}" , trainee);

    println!(" ------------ Structs ------------- ");

    let mut portfo = PorfolioAccount::new("bacem porfolio ", 0.0f64, 0.0f64);

    let  p: &mut dyn Wallet = &mut portfo;

    p.deposit(2.3);

    p.deposit_multiple(&[ 1.0 ,2.0 ,3.0]);

    println!(" porfolio de Bacem est : {}", p.get_bitcoin());

    let p: PorfolioAccount<f64> = PorfolioAccount::new("Amal porfolio ", 6.1f64, 8.3f64);

    println!(" porfolio de Bacem est : {}", p.get_bitcoin());

   /* let mut a = 12;

    let b = 18;

    if a < b {
        println!("a is less than b");
    } else {
        println!(" a is greater than b");
    }

    while a < b {
        print!("{} " , a);
        a += 1;
    } */

    //panic!(" There is a problem here we have to exit");
}

pub enum Trainee {
    Bacem(u32),
    Amal(String),
    Jawaher(bool),
}
use std::ops::Add;

fn sum<T: Add<Output = T>>(x1: T, x2: T) -> T {
    let sum = x1 + x2;
    sum
}

fn print_box(b: Box<u32>) -> Box<u32> {
    println!("{} ", b);
    b
}

fn print_slice(v: &[u32]) {
    for i in v {
        print!("{} ", i);
    }
}

enum Currency {
    BITCOIN,
    ETHEREUM,
}

struct PorfolioAccount<T: Add<Output = T>> {
    name: String,
    values: [(Currency, T); 2],
}

impl Display for PorfolioAccount<f64>
 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(
            f,
            " Porfolio  : name = {} , BITCOIN = {} , ETH = {}",
            self.name, self.values[0].1, self.values[1].1
        );
        Result::Ok(())
    }
}

impl PorfolioAccount<f64> {
    pub fn new(name: &str, bitcoin: f64, eth: f64) -> PorfolioAccount<f64> {
        PorfolioAccount::<f64>{
            name: name.to_string(),
            values: [(Currency::BITCOIN, bitcoin), (Currency::ETHEREUM, eth)],
        }
    }

}



pub struct Employee(String , String);

pub struct Sqoin;


pub trait Wallet {

    fn deposit(&mut self , value: f64) -> ();

    fn deposit_multiple(&mut self , values: &[f64]) {
        for v in values {
            self.deposit(*v);
        }
    }

    fn widhdraw(&mut self , value: f64) -> ();

    fn get_bitcoin(&self) -> f64;

}

impl Wallet for PorfolioAccount<f64> {
    fn deposit(&mut self , value: f64) -> () {
        let mut old = self.values[0].1;
        old += value;
        self.values[0] = (Currency::BITCOIN, old);
    }

    fn widhdraw(&mut self , value: f64) -> () {
        let mut old = self.values[0].1;
        old -= value;
        self.values[0] = (Currency::BITCOIN, old);
    }

    fn get_bitcoin(&self) -> f64 {
        self.values[0].1
    }

} 
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
        print!(" {} " , i);
    }

   
    println!();

    println!("--------- Memory Management Variable --------");

    let m1: u32 = 8u32;

    let m2 = m1;

    println!(" {} {} " , m1 , m2);

    let mut b1 = Box::new( 10u32);

    let b2 = b1;
     b1 = Box::new( 11u32);

    print_box(b1);
    b1 = print_box(b2);
    print_box(b1);
}

fn print_box(b: Box<u32> ) -> Box<u32>  {
    println!("{} " , b);
     b
}


mod sqoin_module {
    pub mod sqoin_sub_module {
    pub struct Sqoiner {
        pub number: i32,
    }
    
    
    pub trait SqoinRules {
        fn shownumber(sqoiner: & Sqoiner) ; 
    }
    
    
    pub fn print_sqoiner(sqoiner: &Sqoiner) -> ()
     {
         println!(" Printing Sqoin {}" , sqoiner.number);
     }
    }
}


mod karima {

    pub fn say_hello() {
        println!(" I am Karima");
    }
}


mod khouloud;
mod jawaher;

fn main() {
    
    sqoin_module::sqoin_sub_module::print_sqoiner(&sqoin_module::sqoin_sub_module::Sqoiner{number: 10});

    println!("-----------------");
 
    karima::say_hello();
    khouloud::say_hello();
    jawaher::jawaherimpl::say_hello(); 


    let x = 100u32 + ( 10u8 as u32);

     let y:u32 =  1000u32;

     let f:f32 = 0.001f32;

    let b: bool =  {
            let mut ret = false;
            if x < y {
                ret = true;
            }
            ret
      };

    let func = move |z:u32| {
        let mut s = x;
        s = s * z *z;
        s
    };
    println!(" value of x is {}" , x);
    println!(" value of y is {}" , y);
    println!(" value of f is {}" , f);
    println!(" value of b is {}" , b);
    println!(" value of func(y) is {}" , func(y));
}

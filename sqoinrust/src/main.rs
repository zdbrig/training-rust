

pub struct Sqoiner {
    number: i32,
}


pub trait SqoinRules {
    fn shownumber(sqoiner: & Sqoiner) ; 
}


pub fn print_sqoiner(sqoiner: &Sqoiner) -> ()
 {
     println!(" Printing Sqoin {}" , sqoiner.number);
 }


fn main() {
    
    print_sqoiner(&Sqoiner{number: 10});

}

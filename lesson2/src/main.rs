
pub trait Wallet<T,U> 
where T: Clone + Sized + Copy , U: Clone
{
    type ReturnType;
    type ReturnType2;

    fn show_wallet(&self) -> Self::ReturnType;
}

impl  Wallet<u32,u32> for u32 {
    type ReturnType =  u64;
    type ReturnType2 = u64;
     fn show_wallet(&self) -> u64 {
         10u64
    }

}

fn main() {
   
    let rust_group = vec![ "Bacem" , "Karima" , "Amal" , "Jawaher" , "Halima" , "Khouloud"];
    
   /* for member in &rust_group {
        println!("{}" ,  member);
    }*/

    let mut iter = (&rust_group).into_iter();
    
    let value = iter.next();
    let real_value = match value {
        Some(v) => v,
        None => ""
    };

    println!("{}" , real_value);


    while let Some(v) = iter.next() {
        println!("{}" , v);
    }
    
}

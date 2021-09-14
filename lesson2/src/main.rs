
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
    println!("Hello, world!");

    let x = 10u32;

    let mut y = x.show_wallet();
    y = Wallet::show_wallet(&x);
    println!("{} " , y);
}

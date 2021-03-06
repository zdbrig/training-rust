pub trait Wallet<T, U>
where
    T: Clone + Sized + Copy,
    U: Clone,
{
    type ReturnType;
    type ReturnType2;

    fn show_wallet(&self) -> Self::ReturnType;
}

impl Wallet<u32, u32> for u32 {
    type ReturnType = u64;
    type ReturnType2 = u64;
    fn show_wallet(&self) -> u64 {
        10u64
    }
}

fn main() {
    let rust_group = vec!["Bacem", "Karima", "Amal", "Jawaher", "Halima", "Khouloud"];
    let other_group = vec!["Ameni", "Emna", "Wafa", "Fedia", "Ameni", "Saif" ,  "Tawfik" ];

    /* for member in &rust_group {
        println!("{}" ,  member);
    }*/

    let mut iter = (&rust_group).into_iter();
    let value = iter.next();
    let real_value = match value {
        Some(v) => v,
        None => "",
    };

    println!("{}", real_value);

    while let Some(v) = iter.next() {
        println!("{}", v);
    }

    println!("************ map *************");

    let y = rust_group.iter().map(|m| (String::new() + m + " kind"));
    for member in y {
        println!("{}", member);
    }

    println!("************ filter *************");

    let y = rust_group.iter().filter(|m| m.len() <= 5);

    for member in y {
        println!("{}", member);
    }
    println!("************ filter map *************");

    let z = rust_group.iter().filter_map(|m| {
        if m.len() > 5 {
            return Some(String::new() + m + " kind ");
        }
        None
    });
    for m1 in z {
        println!("{}", m1);
    }
    println!("************ flat map *************");

    let z = rust_group.iter().flat_map(|m| {
        if m.len() > 5 {
            return Some([m, " kind "]);
        }
        None
    });
    for member in z {
        print!("[");
        for m1 in member {
            print!("{} ", m1);
        }
        print!("]");
    }

    //let it: Vec<u32> = (1..25).collect();

    println!("************ scan *************");

    let iter = rust_group
        .iter()
        .scan(" Sqoin members are ".to_string(), |s, item| {
            s.push_str(item);
            s.push_str(" ");
            Some(s.clone())
        });
    for m1 in iter {
        println!("{}", m1);
    }

    println!("************ take *************");

    let iter = rust_group.iter().take(3);
    for m1 in iter {
        println!("{}", m1);
    }

    println!("************ take while *************");

    let iter = rust_group.iter().take_while(|m| m.len() > 4);
    for m1 in iter {
        println!("{}", m1);
    }

    println!("************ skip *************");

    let iter = rust_group.iter().skip(3);
    for m1 in iter {
        println!("{}", m1);
    }

    println!("************ skip while *************");

    let iter = rust_group.iter().skip_while(|m| m.len() > 4);
    for m1 in iter {
        println!("{}", m1);
    }

    println!("************ peekable *************");

    let mut x = rust_group.iter().peekable();

    if let Some(r) = x.peek() {
        println!("{}", r);
    }
    if let Some(r) = x.next() {
        println!("{}", r);
    }
    if let Some(r) = x.peek() {
        println!("{}", r);
    }
    if let Some(r) = x.peek() {
        println!("{}", r);
    }


    println!("************ Fuse *************");

    let mut iter = (&rust_group).into_iter().fuse();
    
    while let Some(v) = iter.next() {
        println!("{}", v);
    }

    let x = iter.next();
    if let Some(r) = x {
        println!("{}", r);
    } else {
        println!("{}", "None");
    }
    let x = iter.next();
    if let Some(r) = x {
        println!("{}", r);
    } else {
        println!("{}", "None");
    }

    println!("************ Reverse *************");

    let mut iter = (&rust_group).into_iter();
    
     
    while let Some(v) = iter.next_back() {
        println!("{}", v);
    }
    let mut iter = (&rust_group).into_iter().rev();
    
    println!("---------");
     
    while let Some(v) = iter.next() {
        println!("{}", v);
    }

    use std::iter::Inspect;
    println!("************ Inspect *************");

    let mut iter = (&rust_group).into_iter();
    "bacem".chars().inspect(|m| println!(" inspecting {}" , m)).flat_map(|c| c.to_uppercase())
    .inspect(|m| panic!(" can not be here"));

    println!("************ chain *************");

    let mut chain  = rust_group.iter().chain(other_group.iter());
    while let Some(v) = chain.next() {
        println!("{}", v);
    }

    println!("************ enumerate *************");

    for (i, m) in (&rust_group).into_iter().enumerate() {
        println!("{} {}", i , m);
    }

    println!("************ zip *************");
    
    let mut zip  = rust_group.iter().zip(other_group.iter());
    while let Some(v) = zip.next() {
        println!("{} {}", v.0 , v.1 );
    }

    println!("************ by ref *************");

    for i in (&rust_group).into_iter().by_ref() {
        println!("{} ", i );
    }
    println!("************ cloned *************");

    for i in (&rust_group).into_iter().cloned() {
        println!("{} ", i );
    }

    println!("************ cycle *************");

    for (i,j) in (&rust_group).into_iter().cycle().enumerate() {
        println!("{} ", j );
        if i > 10 {
            break 
        }
    }

    println!("************ consuming iterators *************");

    let mut numbers:Vec<u32> = vec![ 1 , 2 , 5 , 7 , 10];

    let  numbes_iter = (&numbers).into_iter();
    
    println!(" sum = {}" , numbes_iter.sum::<u32>() );

    let  numbes_iter = (&numbers).into_iter();

    println!(" product = {}" , numbes_iter.product::<u32>() );

    let  numbes_iter = (&numbers).into_iter();

    println!(" count = {}" , numbes_iter.count() );

    let  numbes_iter = (&numbers).into_iter();

    println!(" min = {}" , numbes_iter.min().unwrap() );

    let mut numbes_iter = (&numbers).into_iter();

    println!(" any  > 8 = {}" , numbes_iter.any(|m| m > &8u32 ) );

    let mut numbes_iter = (&numbers).into_iter();

    println!(" all  > 8 = {}" , numbes_iter.all(|m| m > &8u32 ) );

    let mut numbes_iter = (&numbers).into_iter();

    println!(" position  de  7 = {}" , numbes_iter.position(|m| m > &5000u32 ).unwrap_or(0) );

    let  numbes_iter = (&numbers).into_iter();

    println!(" fold   = {}" , numbes_iter.fold(10, |x , y| x+y ) );
    
    let mut  numbes_iter = (&numbers).into_iter();

    println!(" nth   = {}" , numbes_iter.nth(3 ).unwrap_or(&0u32) );

    let mut  numbes_iter = (&numbers).into_iter();

    println!(" last   = {}" , numbes_iter.last( ).unwrap_or(&0u32) );

    let mut numbes_iter = (&numbers).into_iter();

    println!(" position  de  7 = {}" , numbes_iter.find(|m| m > &&5u32 ).unwrap_or(&0u32) );

    println!("************ Extend *************");

    numbers.extend(0..5);

    for i in &numbers {
        println!("{} " , i);
    }
    println!("************ Partition *************");

    let mut chain  = rust_group.iter().chain(other_group.iter());

    let (p1,p2) : (Vec::<&str> ,Vec::<&str> )= chain.partition(|n| n.len() <= 4);

    for i in p1 {
        println!(" partition 1 {}" , i);
    }

    for i in p2 {
        println!(" partition 2 {}" , i);
    }


}

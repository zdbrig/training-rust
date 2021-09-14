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

    use std::iter::Peekable;
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

}

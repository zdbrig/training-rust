fn main() {
    {
        let name: &str = "bacem";

        let mut sname = String::new();
        sname.push_str(name);

        let c: char = 'b';
        let d: char = '5';

        println!("{} is numeric {}", c, c.is_numeric());
        println!("{} is numeric {}", d, d.is_numeric());

        let e = d.to_digit(10);
        println!(
            "{} to digit {}",
            d,
            e.or_else(|| panic!("invalid number")).unwrap()
        );

        let x = char::from_digit(15u32, 16);
        println!(
            "{} from digit {}",
            5,
            x.or_else(|| panic!("invalid number")).unwrap()
        );
    }

    {
        let name: &str = "Bacem";

        let mut sname = String::new();
        sname.push_str("Bergaoui");
        
        let name_to_string:String = name.to_string();

        let charachter: [char; 4] = ['A' , 'm' , 'a' , 'l'];
        use std::slice::Iter;
        let iter: Iter::<char>= charachter.iter();

        let collection: String = iter.collect();

        println!(" {} " , collection);

        println!("len = {}" , name.len());

        println!("is_empty = {}" , name.is_empty());


    }
}

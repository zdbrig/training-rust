        use regex::Regex;
        extern crate regex;

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

        let name_to_string: String = name.to_string();

        let charachter: [char; 4] = ['A', 'm', 'a', 'l'];
        use std::slice::Iter;
        let iter: Iter<char> = charachter.iter();

        let collection: String = iter.collect();

        println!(" {} ", collection);

        println!("len = {}", name.len());

        println!("is_empty = {}", name.is_empty());

        println!("range = {}", &name[1..4]);

        println!("sname = {}", &sname);

        sname.push('1');

        println!("sname = {}", &sname);

        sname.extend(name.chars());

        println!("sname = {}", &sname);

        sname.insert(8, '✅');

        println!("sname = {}", &sname);

        sname.insert_str(8, "😁");
        println!("sname = {}", &sname);

        sname.clear();
        println!("sname = {}", &sname);

        sname.push_str("Jawaher Korbosli");
        sname.truncate(8);
        println!("sname = {}", &sname);

        sname.pop();
        sname.pop();
        println!("sname = {}", &sname);
        sname.remove(2);
        println!("sname = {}", &sname);
        sname.clear();
        sname.push_str("Jawaher Korbosli");
        sname.drain(4..8);
        println!("sname = {}", &sname);

        sname.clear();
        sname.push_str("Khouloud Hadj Taieb");

        let f = sname.find("dj");
        println!("found at = {}", f.unwrap_or(0));
        println!("contains  {}", sname.contains("mloud"));

        let imen = sname.replace("Khouloud", "Imen");
        println!("sname = {}", &imen);

        let name = "Karima Zouaoui";

        for i in name.chars() {
            print!(" {} ", i);
        }
        println!();
        for (i, j) in name.char_indices() {
            print!(" [{},{}] ", i, j);
        }
        println!();
        let name = "Karima 🚗 Zouaoui";
        for i in name.bytes() {
            print!(" {} ", i);
        }
        let name = "   Karima      Zouaoui \n Amal         Fathallah";
        println!();
        for i in name.lines() {
            println!(" {} ", i);
        }

        let v: Vec<&str> = name.lines().collect();

        let newname = name.trim();
        println!("newname = {}", &newname);

        use std::str::FromStr;

        let b: bool = bool::from_str("true").unwrap_or(false);

        let f: f64 = f64::from_str("12.2234244").unwrap_or(1.2);

        let i = 123i32;

        let v = vec![1 , 2 , 4 , 5];
        println!("i = {1} f={0:#^50} " , f , i);

        println!("v = {:?} " , v );

        println!("value = {} , pointer = {:p} " , &sname , &sname );
        let sname2 = sname.clone();
        println!("value = {} , pointer = {:p} " , &sname2 , &sname2 );

        
        let pattern = Regex::new(r"^a+$").unwrap();

        println!("{} {} {}" , pattern.is_match("a") , pattern.is_match("aaaa") ,
        pattern.is_match("b"));

        let names = r" Khouloud bacem Halima Jawaher Amal Karima ";

        let pattern = Regex::new(r" [a-zA-Z]*a[a-zA-Z]* ").unwrap();
        println!("{} " , pattern.is_match(names));

        let captures = pattern.find_iter(names);

       // println!("{}" , captures.get(0).unwrap().start());
        for (i , j ) in captures.enumerate() {
            println!(" {} -  {} " , i , j.as_str());
        }

    }
}

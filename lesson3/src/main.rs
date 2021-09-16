fn main() {
    let mut v = Vec::<u32>::with_capacity(10);
    let mut v2 = Vec::<u32>::with_capacity(10);

    v.extend(1..14);

    let boxes: Vec<Box<&u32>> = (&v).into_iter().map(|a| Box::new(a)).collect();

    let x = v[0];

    let b = &boxes[0];

    let vclone = (&v).clone();

    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }
    {
        let mut sliceref = &(&v)[5..11];
        println!("-----------");

        for (i, j) in sliceref.into_iter().enumerate() {
            println!(" {} = {} ", i, j);
        }

        println!(" first is {}", sliceref.first().unwrap_or(&0u32));
        println!(" last is {}", sliceref.last().unwrap_or(&0u32));
        println!(" number 4 is {}", sliceref.get(4).unwrap_or(&0u32));
    }
    v.push(77u32);

    println!("-----------");

    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }

    v.pop();

    println!("-----------");

    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }

    v.insert(0, 77u32);
    println!("-----------");

    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }
    v.drain(1..4);
    println!("-----------");

    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }

    let table = [[1, 2, 77], [3, 4, 88], [5, 6, 99]].join(&15);
    for (i, j) in (&table).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }

    use std::collections::VecDeque;
    let mut vq: VecDeque<u32> = VecDeque::new();
    vq.push_front(10u32);
    println!("**********************");
    v2.extend(1..5);
    v.append(&mut v2);
    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }
    println!("************DRAIN**********");

    let res = v.drain(0..3);
    for (i, j) in res.into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }
    println!("**** V ***");
    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }

    println!("************ retrain **********");
    v.retain(|x| x < &5);
    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }
    println!();
    println!("************ dedup **********");
    let mut byte_vec = [1, 2, 2, 3, 3, 5, 4].to_vec();
    {
        byte_vec.dedup();
        for i in (&byte_vec).into_iter() {
            print!(" {}  ", i);
        }
    }
    println!();
    println!("************ dedup-by **********");
    byte_vec.dedup_by(|x, y| x == y);
    for i in (&byte_vec).into_iter() {
        print!(" {}  ", i);
    }
    println!();
    println!("************ dedup-by_key **********");

    let s: Sqoin = Sqoin {
        nom: "jawaher".to_string(),
        cin: 1,
    };
    let s2: Sqoin = Sqoin {
        nom: "jawaher".to_string(),
        cin: 1,
    };
    let s3: Sqoin = Sqoin {
        nom: "jawher".to_string(),
        cin: 2,
    };
    let s4: Sqoin = Sqoin {
        nom: "jawaher".to_string(),
        cin: 2,
    };
    let mut struct_vec = [&s, &s2, &s3, &s4].to_vec();

    struct_vec.dedup_by_key(|x| x.cin);
    for i in struct_vec.into_iter() {
        println!(" {}  ", i.cin);
    }

    println!("************ slice concat **********");
    let table = [[1, 2, 77], [3, 4, 88], [5, 6, 99]].concat();
    for (i, j) in (&table).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }
    println!("************ Splitting **********");

    let v = vec![0, 1, 2, 3, 2, 4, 3, 4, 5, 6, 6];
    let a = &v[1];
    let b = &v[2];

    let mid = v.len() / 2;
    let front_half = &v[..mid];
    let back_half = [mid..];

    let mut v1 = vec![0, 1, 2, 3];
    let a = &mut v1[1];
    let b = &mut v1[2];

    //let x = v.iter();
    // let y = v1.iter();
    for i in v1.iter_mut() {
        println!(" {}  ", i);
    }

    println!("********* split_at *******");
    let (res1, res2) = v1.split_at(2);
    //println!(" res = {}", res);

    for i in res1.iter() {
        println!(" res left = {}  ", i);
    }
    for i in res2.iter() {
        println!(" res right = {}  ", i);
    }

    println!("********* split_first *******");

    if let Some((first, elements)) = v.split_first() {
        println!("{}  ", first);
        for (i, j) in elements.into_iter().enumerate() {
            println!("{} = {}", i, j);
        }
    }

    println!("********* split_last *******");

    if let Some((last, elements)) = v.split_last() {
        println!("{}  ", last);
        for (i, j) in elements.into_iter().enumerate() {
            println!("{} = {}", i, j);
        }
    }

    println!("*********split *******");

    let res = v.split(|m| m == &3u32);

    for i in res.into_iter() {
        println!("---");
        for j in i.into_iter() {
            println!("{}", j);
        }
    }

    println!("*********split n *******");

    let res = v.splitn(2, |m| m == &3u32);

    for i in res.into_iter() {
        println!("---");
        for j in i.into_iter() {
            println!("{}", j);
        }
    }

    println!("********* rsplit n *******");

    let res = v.rsplitn(2, |m| m == &3u32);

    for i in res.into_iter() {
        println!("---");
        for j in i.into_iter() {
            println!("{}", j);
        }
    }

    println!("********* chunks *******");

    let res = v.chunks(4);

    for i in res.into_iter() {
        println!("---");
        for j in i.into_iter() {
            println!("{}", j);
        }
    }

    println!("********* windows *******");

    let res = v.windows(4);

    for i in res.into_iter() {
        println!("---");
        for j in i.into_iter() {
            println!("{}", j);
        }
    }
    println!("********* Swapping *******");
    println!("-------- Swap---------");
    let mut vec = [8, 2, 3, 5, 7,1];
    vec.swap(1, 3);
    for (i, j) in vec.iter().enumerate() {
        println!("{} = {}", i, j);
    }
    println!("-------- Swap-Remove ---------");
    let mut v = vec!["bacem", "karima", "khouloud", "jawaher"];
    v.swap_remove(2);
    for (i, j) in v.iter().enumerate() {
        println!("{} = {}", i, j);
    }
    println!("********* Sorting and Searching *******");
    println!("-------- sort---------");
    v1.sort();
    for (i, j) in v1.iter().enumerate() {
        println!("{} = {}", i, j);
    }
    println!("-------- sort by---------");

    use std::collections::BTreeMap;
    use std::collections::BTreeSet;
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::collections::LinkedList;

    // get mut to make a modification
}

pub struct Sqoin {
    nom: String,
    cin: u64,
}

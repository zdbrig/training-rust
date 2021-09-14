fn main() {
    let mut v = Vec::<u32>::with_capacity(10);

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

    v.insert(0 , 77u32);
    println!("-----------");

    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }
    v.drain(1..4);
    println!("-----------");

    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }

    let table = [ [1,2 , 77] , [3,4 , 88] , [5,6 , 99] ].join(&15);
    for (i, j) in (&table).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }

    use std::collections::VecDeque;
    let mut vq: VecDeque::<u32> = VecDeque::new();
    vq.push_front(10u32);

    use std::collections::LinkedList;
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    use std::collections::BTreeMap;
    use std::collections::HashSet;
    use std::collections::BTreeSet;






    // get mut to make a modification
}

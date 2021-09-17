use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::LinkedList;
fn main() {
    println!("************collection*******");

    let mut vec:Vec<u32>=vec![1,2,3,4,5,6];
    vec.push(7);
    for i in vec.iter_mut(){
        println!("{}",i);
    }
println!("lenght {}",vec.len());
println!("first element {}",vec[0]);
println!("last  element {:?}",vec.pop());
vec[0]=10;
println!("read  element {}",vec[0]);
vec.extend([1,9,6].iter().copied());
for i in &vec{
    println!("new element vec{}",i);
}
vec.extend(vec![12,19,16]);
for i in &vec{
    println!("{}",i);
}
&vec.insert(5, 20);
for i in vec{
    println!("new element vec{}",i);
}

println!("************vecDeque*******");
let mut deq :VecDeque<u32>= VecDeque::new();
deq.push_back(20);
deq.push_back(2);
deq.push_front(100);
deq.push_back(10);
deq.push_back(5);
println!("{:?}",deq);
println!("index{:?}",deq.get(3));
println!("index{:?}",deq.get_mut(1));
deq.swap(2, 3);

println!(" after swap {:?}",deq);
println!("capacity{}",deq.capacity());

println!("***********linkdlist********");
let mut linkdlist: LinkedList<u32> = LinkedList::new();
let mut linkdlist1: LinkedList<u32> = LinkedList::new();
linkdlist.push_back(5);
linkdlist.push_back(7);
linkdlist.push_back(8);
linkdlist.push_front(10);
linkdlist1.push_back(5);
linkdlist1.push_back(7);
linkdlist1.push_back(8);
linkdlist1.push_front(100);
linkdlist.append(&mut linkdlist1);
println!(" linkdlist {:?}",linkdlist);
println!("***********HASHmAP********");
let mut sqoin = HashMap::new();
sqoin.insert(
    "name",
    "khouloud".to_string(),
  
);
sqoin.insert(
   
    "name",
    "karima".to_string(),
);
println!(" member sqoin {:?}",sqoin);
println!(" lenght sqoin {:?}",sqoin.len());
sqoin.remove("name");
println!(" remove member sqoin {:?}",sqoin.len());
}

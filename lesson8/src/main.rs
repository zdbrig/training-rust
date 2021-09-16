use std::io;
use std::ops::Range;


#[derive(PartialEq, Clone,Debug)]
struct Foo<T> {
    a: i32,
    b: T,
}
#[cfg(test)]
pub mod m1 {
    // Missing documentation is ignored here
    #[allow(missing_docs)]
    pub fn undocumented_one() -> i32 { 1 }

    // Missing documentation signals a warning here
    #[warn(missing_docs)]
    #[doc="halima"]
    #[test]
    pub fn undocumented_too() { assert_eq!(2,2) }

    // Missing documentation signals an error here
    #[deny(missing_docs)]
    #[doc="halima"]
    pub fn undocumented_end() -> i32 { 3 }
}
#[warn(missing_docs)]
#[doc="halima"]
pub mod m2{
    #[doc="halima"]
    #[allow(missing_docs)]
    pub mod nested {
        // Missing documentation is ignored here
        pub fn undocumented_one() -> i32 { 1 }

        // Missing documentation signals a warning here,
        // despite the allow above.
        #[doc="halima"]
        #[warn(missing_docs)]
        pub fn undocumented_two() -> i32 { 2 }
    }

    // Missing documentation signals a warning here
    pub fn undocumented_too() -> i32 { 3 }
}

//#[forbid(missing_docs)]
pub mod m3 {
    // Attempting to toggle warning signals an error here
    
    #[allow(missing_docs)]
    /// Returns 2.
    pub fn undocumented_too() -> i32 { 2 }
}


fn main() {
  
  let f:Foo<u32>=Foo{ a:12,b:5};
  let f1:Foo<u32>=Foo{ a:12,b:5};
  let res=f==f1;
  let  x=f.clone();
  println!("{:?}",res);
}
/// assert_eq!(ranges::overlap(0..7,3..10),true);
///assert_eq!(ranges::overlap(1..5,101..105),false);
pub fn overlap(r1:Range<usize>,r2:Range<usize>)->bool{
    r1.start <r1.end && r2.start<r2.end && r1.start<r2.end &&r2.start<r1.end

}


fn print1() -> u32{
    return 1;
}

#[test]
fn test_print1(){
    assert_eq!(print1(), 1);
}
#[test]
#[ignore = "not yet implemented"]
fn test_print2(){
    assert_eq!(print1(), 2);
}

#[test]
#[should_panic(expected = "values don't match")]
fn mytest() {
    assert_eq!(1, 2, "values don't match");
}


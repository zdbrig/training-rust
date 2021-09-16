
use serde::{Serialize, Deserialize};

use std::marker::PhantomData;

#[derive(Clone , Copy,Serialize, Deserialize )]
pub enum VmState{
    STOPPED,
    STARTED,
    PAUSED
}

#[macro_export]
macro_rules!  print_virtual{
    ($a: expr) => {
        println!("####{} {} ####" , $a.name , " this is a state");
    };
}

#[derive(Clone,Copy ,Serialize, Deserialize)]
pub struct VirtualMachine<'a , T>
    where T: ?Sized + Clone + Copy
{
    pub name: &'a str,
    pub state: VmState,
    pub resource_type: PhantomData<T>,
}


use std::marker::PhantomData;
pub enum VmState{
    STOPPED,
    STARTED,
    PAUSED
}


pub struct VirtualMachine<T>
    where T: ?Sized
{

    pub name: String,
    pub state: VmState,
    pub resource_type: PhantomData<T>,
}
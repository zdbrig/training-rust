use crate::virtual_machine::VirtualMachine;
use serde::{Serialize, Deserialize};

mod data_center;
pub mod virtual_machine;
pub mod vm_trait;
pub mod vm_trait_controller;
pub mod linux;
pub mod macos;
pub mod datacenterimpl;
mod vm_type;

fn main() {

    use  vm_trait::VmTrait;
    use linux::Linux;
    use macos::MACOS;
    let mut o:VirtualMachine::<MACOS> =   VmTrait::<MACOS>::new();
    print_virtual!(o);

    let serialized = serde_json::to_string(& o).unwrap();
    println!("##serialized = {}##", serialized);

    println!("{}" , o.info());
    o.start();
    println!("{}" , o.info());
    o.pause();
    println!("{}" , o.info());

    let mut o:VirtualMachine::<Linux> =   VmTrait::<Linux>::new();
    
    println!("{}" , o.info());
    o.start();
    println!("{}" , o.info());
    o.pause();
    println!("{}" , o.info());



}

use crate::vm_type::VmType;

pub trait VmTrait<VM> {

    fn start(&mut self);

    fn stop(&mut self);

    fn pause(&mut self);

    fn new() -> Self;

    fn info(&self) -> String;
}
use crate::vm_trait_controller::VmTraitController;
use crate::vm_trait::VmTrait;



pub struct DataCenter<'a, T> {

    name: String,
    vms: Vec::<&'a dyn VmTraitController<T>> ,
}
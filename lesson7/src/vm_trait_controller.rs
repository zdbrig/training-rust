
use crate::vm_trait::VmTrait;


pub trait VmTraitController<V> 
    where V: VmTrait::<V>
{
    fn start_vm(&self, v: V);
    fn stop_vm(&self, v: V);
    fn pause_vm(&self, v: V);
}
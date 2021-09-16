use crate::linux::Linux;
use crate::macos::MACOS;
use crate::virtual_machine::VirtualMachine;
use crate::virtual_machine::VmState;
use crate::vm_trait::VmTrait;
use rand::Rng;

impl VmTrait<Linux> for VirtualMachine<Linux> {

    fn start(&mut self) {
        //  self.start_linux();
        Linux::start_linux();
        self.state = VmState::STARTED;
    }

    fn stop(&mut self) {
        Linux::stop_linux();
        self.state = VmState::STOPPED;
    }

    fn pause(&mut self) {
        Linux::pause_linux();
        self.state = VmState::PAUSED;
    }

    fn new() -> VirtualMachine<Linux> {
        let mut rng = rand::thread_rng();
        let number: u32 = rng.gen();
        VirtualMachine::<Linux>  {
            name: format!("LINUX_{}", number),
            state: VmState::STOPPED,
            resource_type: std::marker::PhantomData,
        }
    }

    fn info(&self) -> String {
        let x = match self.state {
            VmState::STARTED => "Started",
            VmState::STOPPED => "Stopped",
            VmState::PAUSED => "Paused",
        };
        format!(" MACHINE {} STATUS {}", self.name, x)
    }
}

impl VmTrait<MACOS> for VirtualMachine<MACOS> {

    fn start(&mut self) {
        //  self.start_linux();
        MACOS::start_macos();
        self.state = VmState::STARTED;
    }

    fn stop(&mut self) {
        MACOS::stop_macos();
        self.state = VmState::STOPPED;
    }

    fn pause(&mut self) {
        MACOS::pause_macos();
        self.state = VmState::PAUSED;
    }

    fn new() -> VirtualMachine<MACOS> {
        let mut rng = rand::thread_rng();
        let number: u32 = rng.gen();
        VirtualMachine::<MACOS> {
            name: format!("MACOS_{}", number),
            state: VmState::STOPPED,
            resource_type: std::marker::PhantomData,

        }
    }

    fn info(&self) -> String {
        let x = match self.state {
            VmState::STARTED => "Started",
            VmState::STOPPED => "Stopped",
            VmState::PAUSED => "Paused",
        };
        format!(" MACHINE {} STATUS {}", self.name, x)
    }
}




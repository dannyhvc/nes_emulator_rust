use crate::debug::types::utilities::Utilities;

#[derive(Debug, Clone)]
pub struct Debuggees {
    pub cpu: crate::components::dh_cpu::cpu::CPU,
    pub bus: crate::components::dh_bus::bus::BUS,
    pub util: Utilities,
}

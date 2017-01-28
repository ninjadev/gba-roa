use interconnect;
use cpu;

pub struct GameBoyAdvance {
    pub cpu: cpu::ARM7TDMI,
    pub interconnect: interconnect::Interconnect,
}

impl GameBoyAdvance {
    pub fn new() -> Self {
        GameBoyAdvance {
            cpu: cpu::ARM7TDMI::new(),
            interconnect: interconnect::Interconnect::new(),
        }
    }
}
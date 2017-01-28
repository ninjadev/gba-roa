use opcode::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Shifted {
        opcode: OpcodeShifted,
        offset: u8,
        source_register: u8,
        destination_register: u8,
    },
    AddSub {
        opcode: OpcodeAddSub,
        operand: u8,
        source_register: u8,
        destination_register: u8,
    },
    Immediate {
        opcode: OpcodeImmediate,
        destination_register: u8,
        unsigned_immediate: u8,
    },
    AluOperation {
        opcode: OpcodeAluOperation,
        source_register: u8,
        destination_register: u8,
    },
}

use instruction::Instruction;
use instruction::Instruction::AddSub;

#[derive(Debug)]
pub struct ARM7TDMI {
    reg_gpr: [u32; 16],

    cprs: PSR,

    reg_banked: [[u32; 7]; 6],
    sprs_banked: [PSR; 6],

    privilege_mode: PrivilegeMode,
    execution_mode: ExecutionMode,
}

impl ARM7TDMI {
    pub fn new() -> ARM7TDMI {
        let reg_gpr = [0x00000011; 16];
        let reg_banked = [[0xdeadbeef; 7]; 6];
        let sprs_banked = [PSR {
            n: false,
            z: false,
            c: false,
            v: false,
            i: false,
            f: false,
            t: false,
        }; 6];

        ARM7TDMI {
            reg_gpr: reg_gpr,
            cprs: PSR {
                n: false,
                z: false,
                c: false,
                v: false,
                i: false,
                f: false,
                t: false,
            },
            reg_banked: reg_banked,
            sprs_banked: sprs_banked,
            privilege_mode: PrivilegeMode::User,
            execution_mode: ExecutionMode::ARM,
        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            AddSub { opcode, operand, source_register, destination_register } => {
                use opcode::OpcodeAddSub::*;
                match opcode {
                    AddRegister => {
                        self.register_instruction(operand,
                                                  source_register,
                                                  destination_register,
                                                  |rs, rn| rs + rn)
                    }
                    SubRegister => {
                        self.register_instruction(operand,
                                                  source_register,
                                                  destination_register,
                                                  |rs, rn| rs - rn)
                    }
                    AddImmediate => {
                        self.immediate_instruction(operand,
                                                   source_register,
                                                   destination_register,
                                                   |rs, nn| rs + nn)
                    }
                    SubImmediate => {
                        self.immediate_instruction(operand,
                                                   source_register,
                                                   destination_register,
                                                   |rs, nn| rs - nn)
                    }
                }
            }
            _ => (),
        }
    }

    pub fn get_registers(&mut self) -> [u32; 16] {
        self.reg_gpr
    }

    fn read_register(&mut self, register: u8) -> u32 {
        self.reg_gpr[register as usize]
    }

    fn write_register(&mut self, register: u8, value: u32) {
        self.reg_gpr[register as usize] = value;
    }

    fn immediate_instruction<F>(&mut self, immediate: u8, source: u8, destination: u8, f: F)
        where F: FnOnce(u32, u32) -> u32
    {
        let rs = self.read_register(source);
        let nn = immediate as u32;

        let result = f(rs, nn);

        self.write_register(destination, result);
    }

    fn register_instruction<F>(&mut self, operand: u8, source: u8, destination: u8, f: F)
        where F: FnOnce(u32, u32) -> u32
    {
        let rs = self.read_register(source);
        let rn = self.read_register(operand);

        let result = f(rs, rn);

        self.write_register(destination, result);
    }
}

#[derive(Debug, Copy, Clone)]
struct PSR {
    n: bool,
    z: bool,
    c: bool,
    v: bool,
    i: bool,
    f: bool,
    t: bool,
}

#[derive(Debug)]
enum PrivilegeMode {
    User = 0x10,
    FIQ = 0x11,
    IRQ = 0x12,
    Supervisor = 0x13,
    Abort = 0x17,
    Undefined = 0x1B,
    System = 0x1F,
}

#[derive(Debug)]
enum ExecutionMode {
    ARM,
    THUMB,
}

#[cfg(test)]
mod tests {
    use super::*;
    use opcode::OpcodeAddSub;

    #[test]
    fn do_instruction() {
        let add_register = Instruction::AddSub {
            opcode: OpcodeAddSub::AddRegister,
            operand: 7,
            source_register: 10,
            destination_register: 0,
        };
        let sub_register = Instruction::AddSub {
            opcode: OpcodeAddSub::SubRegister,
            operand: 7,
            source_register: 10,
            destination_register: 1,
        };
        let add_immediate = Instruction::AddSub {
            opcode: OpcodeAddSub::AddImmediate,
            operand: 7,
            source_register: 10,
            destination_register: 2,
        };
        let sub_immediate = Instruction::AddSub {
            opcode: OpcodeAddSub::SubImmediate,
            operand: 1,
            source_register: 10,
            destination_register: 3,
        };

        let mut cpu = ARM7TDMI::new();
        cpu.execute_instruction(add_register);
        cpu.execute_instruction(sub_register);
        cpu.execute_instruction(add_immediate);
        cpu.execute_instruction(sub_immediate);

        let registers = cpu.get_registers();
        assert_eq!(registers[0], 34); // rd = rs + rn
        assert_eq!(registers[1], 0); // rd = rs - rn
        assert_eq!(registers[2], 24); // rd = rs + nn
        assert_eq!(registers[3], 16); // rd = rs - nn
    }
}

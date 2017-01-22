use nom::*;
use opcode::*;
use enum_primitive::FromPrimitive;

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

named!(parse_thumb1<Instruction>,
    bits!(
        do_parse!(
            tag: tag_bits!(u8, 3, 0b000) >>
            opcode: take_bits!(u8, 2) >>
            offset: take_bits!(u8, 5) >>
            source_register: take_bits!(u8, 3) >>
            destination_register: take_bits!(u8, 3) >>
            (Instruction::Shifted {
                opcode: OpcodeShifted::from_u8(opcode)
                    .expect("Unrecognized opcode"),
                offset: offset,
                source_register: source_register,
                destination_register: destination_register,
            })
        )
    )
);

named!(parse_thumb2<Instruction>,
    bits!(
        do_parse!(
            tag: tag_bits!(u8, 5, 0b00011) >>
            opcode: take_bits!(u8, 2) >>
            operand: take_bits!(u8, 3) >>
            source_register: take_bits!(u8, 3) >>
            destination_register: take_bits!(u8, 3) >>
            (Instruction::AddSub {
                opcode: OpcodeAddSub::from_u8(opcode)
                    .expect("Unrecognized opcode"),
                operand: operand,
                source_register: source_register,
                destination_register: destination_register,
            })
        )
    )
);

named!(parse_thumb3<Instruction>,
    bits!(
        do_parse!(
            tag: tag_bits!(u8, 3, 0b001) >>
            opcode: take_bits!(u8, 2) >>
            destination_register: take_bits!(u8, 3) >>
            unsigned_immediate: take_bits!(u8, 8) >>
            (Instruction::Immediate {
                opcode: OpcodeImmediate::from_u8(opcode)
                    .expect("Unrecognized opcode"),
                destination_register: destination_register,
                unsigned_immediate: unsigned_immediate,
            })
        )
    )
);

named!(parse_thumb4<Instruction>,
    bits!(
        do_parse!(
            tag: tag_bits!(u8, 6, 0b010000) >>
            opcode: take_bits!(u8, 4) >>
            source_register: take_bits!(u8, 3) >>
            destination_register: take_bits!(u8, 3) >>
            (Instruction::AluOperation {
                opcode: OpcodeAluOperation::from_u8(opcode)
                    .expect("Unrecognized opcode"),
                source_register: source_register,
                destination_register: destination_register,
            })
        )
    )
);

named!(parse_thumb<Vec<Instruction>>,
    many0!(
        alt!(
            // thumb2 has more significant bits in match, so has to go first,
            // as not to conflict with thumb1 instruction
            parse_thumb2 |
            parse_thumb1 |
            parse_thumb3 |
            parse_thumb4
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_shifted() {
        let move_shift = vec![ 0b00010101, 0b11001000, ];
        let (_, res) = parse_thumb1(&move_shift).unwrap();
        assert_eq!(res, Instruction::Shifted {
            opcode: OpcodeShifted::ArithmeticShiftRight,
            offset: 23,
            source_register: 1,
            destination_register: 0,
        });
    }

    #[test]
    fn parse_instructions() {
        let hex = vec![
            0b00011101, 0b11001000, // Add immediate
            0b00110111, 0b10000000, // Add immediate 128 to r7
            0b01000001, 0b11101100, // Rotate r4 right by r5
        ];

        let (_, instructions) = parse_thumb(&hex).unwrap();

        let expected_instructions = [Instruction::AddSub {
                                         opcode: OpcodeAddSub::AddImmediate,
                                         operand: 7,
                                         source_register: 1,
                                         destination_register: 0,
                                     },
                                     Instruction::Immediate {
                                         opcode: OpcodeImmediate::Add,
                                         destination_register: 7,
                                         unsigned_immediate: 128,
                                     },
                                     Instruction::AluOperation {
                                         opcode: OpcodeAluOperation::ROR,
                                         source_register: 5,
                                         destination_register: 4,
                                     }];

        assert_eq!(instructions, expected_instructions);
    }
}

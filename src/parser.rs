use nom::*;

#[derive(Debug)]
enum Instruction {
    THUMB1 {
        tag: u8,
        opcode: u8,
        offset: u8,
        source_register: u8,
        destination_register: u8,
    },
    THUMB2 {
        tag: u8,
        opcode: u8,
        operand: u8,
        source_register: u8,
        destination_register: u8,
    },
    THUMB3 {
        tag: u8,
        opcode: u8,
        destination_register: u8,
        unsigned_immediate: u8,
    },
    THUMB4 {
        tag: u8,
        opcode: u8,
        source_register: u8,
        destination_register: u8,
    }
}

named!(parse_thumb1<Instruction>,
    bits!(
        do_parse!(
            tag: tag_bits!(u8, 3, 0b000) >>
            opcode: take_bits!(u8, 2) >>
            offset: take_bits!(u8, 5) >>
            source_register: take_bits!(u8, 3) >>
            destination_register: take_bits!(u8, 3) >>
            (Instruction::THUMB1 {
                tag: tag,
                opcode: opcode,
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
            (Instruction::THUMB2 {
                tag: tag,
                opcode: opcode,
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
            (Instruction::THUMB3 {
                tag: tag,
                opcode: opcode,
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
            (Instruction::THUMB4 {
                tag: tag,
                opcode: opcode,
                source_register: source_register,
                destination_register: destination_register,
            })
        )
    )
);

named!(parse_thumb<Vec<Instruction>>,
    many0!(
        alt!(
            parse_thumb1 |
            parse_thumb2 |
            parse_thumb3 |
            parse_thumb4
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instruction() {
        let move_shift_thumb1 = vec![ 0b00011101, 0b11001000, ];
        let add_immediate_thumb2 = vec![ 0b00011101, 0b11001000, ];
        let add_immediate_128_to_r7 = vec![ 0b00110111, 0b10000000 ];
        let rotate_r4_right_by_r5 = vec![ 0b01000001, 0b11101100 ];
        println!("{:?}", parse_thumb(&rotate_r4_right_by_r5));
    }
}

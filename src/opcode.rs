enum_from_primitive! {
    #[derive(Debug, PartialEq, Eq)]
    pub enum OpcodeShifted {
        LogicalShiftLeft = 0,
        LogicalShiftRight = 1,
        ArithmeticShiftRight = 2,
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq, Eq)]
    pub enum OpcodeAddSub {
        AddRegister = 0,
        SubRegister = 1,
        AddImmediate = 2,
        SubImmediate = 3,
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq, Eq)]
    pub enum OpcodeImmediate {
        Mov = 0,
        Cmp = 1,
        Add = 2,
        Sub = 3,
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq, Eq)]
    pub enum OpcodeAluOperation {
        And = 0x0,
        EOR = 0x1,
        LSL = 0x2,
        LSR = 0x3,
        ASR = 0x4,
        ADC = 0x5,
        SBC = 0x6,
        ROR = 0x7,
        TST = 0x8,
        NEG = 0x9,
        CMP = 0xa,
        CMN = 0xb,
        ORR = 0xc,
        MUL = 0xd,
        BIC = 0xe,
        MVN = 0xf,
    }
}

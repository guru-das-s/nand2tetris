#[derive(Debug)]
pub enum Destination {
    Null = 0,
    M = 1,
    D = 2,
    DM = 3,
    A = 4,
    AM = 5,
    AD = 6,
    ADM = 7,
}

#[derive(Debug)]
pub enum Comp {
    Zero,
    One,
    MinusOne,
    D,
    A,
    M,
    NotD,
    NotA,
    NotM,
    MinusD,
    MinusA,
    MinusM,
    DplusOne,
    AplusOne,
    MplusOne,
    DminusOne,
    AminusOne,
    MminusOne,
    DplusA,
    DplusM,
    DminusA,
    DminusM,
    AminusD,
    MminusD,
    DandA,
    DandM,
    DorA,
    DorM,
}

impl Comp {
    pub fn to_u8(&self) -> u8 {
        match self {
            Comp::Zero => 0b101010,
            Comp::One => 0b111111,
            Comp::MinusOne => 0b111010,
            Comp::D => 0b001100,
            Comp::A | Comp::M => 0b110000,
            Comp::NotD => 0b001101,
            Comp::NotA | Comp::NotM => 0b110001,
            Comp::MinusD => 0b001111,
            Comp::MinusA | Comp::MinusM => 0b110011,
            Comp::DplusOne => 0b011111,
            Comp::AplusOne | Comp::MplusOne => 0b110111,
            Comp::DminusOne => 0b001110,
            Comp::AminusOne | Comp::MminusOne => 0b110010,
            Comp::DplusA | Comp::DplusM => 0b000010,
            Comp::DminusA | Comp::DminusM => 0b010011,
            Comp::AminusD | Comp::MminusD => 0b000111,
            Comp::DandA | Comp::DandM => 0b000000,
            Comp::DorA | Comp::DorM => 0b010101,
        }
    }

    pub fn to_bitfield_a_val(&self) -> u8 {
        match self {
            Comp::M
            | Comp::NotM
            | Comp::MinusM
            | Comp::MplusOne
            | Comp::MminusOne
            | Comp::DplusM
            | Comp::DminusM
            | Comp::MminusD
            | Comp::DandM
            | Comp::DorM => 1,
            _ => 0,
        }
    }
}

#[derive(Debug)]
pub enum Jump {
    Null = 0,
    JGT = 1,
    JEQ = 2,
    JGE = 3,
    JLT = 4,
    JNE = 5,
    JLE = 6,
    JMP = 7,
}

pub enum Destination {
    Null,
    M,
    D,
    DM,
    MD, // Same as DM
    A,
    AM,
    AD,
    ADM,
    AMD, // Same as ADM
}

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

pub enum Jump {
    Null,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

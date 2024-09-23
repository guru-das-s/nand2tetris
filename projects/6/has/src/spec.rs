#[derive(Debug)]
pub enum Destination {
    Null,
    M,
    D,
    DM,
    A,
    AM,
    AD,
    ADM,
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

#[derive(Debug)]
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

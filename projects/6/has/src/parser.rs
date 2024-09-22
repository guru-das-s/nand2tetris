use crate::spec::{Comp, Destination, Jump};

pub enum HackLine {
    Whitespace,
    Comment,
    A {
        value: u16,
    },
    C {
        dest: Destination,
        comp: Comp,
        jump: Jump,
    },
}

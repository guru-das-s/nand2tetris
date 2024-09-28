use crate::parser::HackLine;
use crate::spec::{Comp, Destination, Jump};
use crate::symboltable::SymbolTable;

fn binary_of_a_type_instruction(value: u16) -> String {
    let a_val = value & 0x7FFF; // Set MSB to zero
    let a_val_binary_string = format!("{:016b}", a_val);
    a_val_binary_string
}

fn binary_of_c_type_instruction(dest: Destination, comp: Comp, jump: Jump) -> String {
    let c_instr_opcode = 0b111;

    let a_num: u8 = comp.to_bitfield_a_val();
    let dest_num: u8 = dest as u8;
    let comp_num: u8 = comp.to_u8();
    let jump_num: u8 = jump as u8;

    let c_binary_string = format!(
        "{:03b}{:01b}{:06b}{:03b}{:03b}",
        c_instr_opcode, a_num, comp_num, dest_num, jump_num
    );

    c_binary_string
}

fn binary_of_variable(variable: String, symbol_table: &mut SymbolTable) -> String {
    if !symbol_table.is_known(&variable) {
        symbol_table.add_new_variable(&variable);
    }
    let value = symbol_table.get_variable_address(&variable);
    return binary_of_a_type_instruction(value);
}

pub fn binary_of(line: HackLine, symboltable: &mut SymbolTable) -> Option<String> {
    match line {
        HackLine::Whitespace | HackLine::Comment | HackLine::Label { .. } => None,
        HackLine::A { value } => Some(binary_of_a_type_instruction(value)),
        HackLine::C { dest, comp, jump } => Some(binary_of_c_type_instruction(
            dest.unwrap(),
            comp.unwrap(),
            jump.unwrap(),
        )),
        HackLine::Variable { name } => Some(binary_of_variable(name, symboltable)),
    }
}

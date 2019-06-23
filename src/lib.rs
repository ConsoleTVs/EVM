pub mod opcodes;
pub mod values;

use opcodes::Opcode;
use values::Value;

#[derive(Debug, PartialEq)]
pub enum EVMResult {
    Success,
    Fail(usize)
}

pub fn interpret(code: &[Opcode], constants: &[Value], registers: &mut [Value]) -> EVMResult {
    let mut pc = 0usize;
    loop {
        match code[pc] {
            Opcode::Load(rx, c1) => registers[rx as usize] = constants[c1 as usize].clone(),
            Opcode::Add(rx, ry, rz) => registers[rx as usize] = registers[ry as usize].add(&registers[rz as usize]),
            Opcode::Sub(rx, ry, rz) => registers[rx as usize] = registers[ry as usize].sub(&registers[rz as usize]),
            Opcode::Print(rx) => println!("{}", registers[rx as usize].to_string()),
            Opcode::Exit => break,
        }
        pc += 1;
    }
    EVMResult::Success
}

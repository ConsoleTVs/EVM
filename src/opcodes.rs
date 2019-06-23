/// Constant
type Reg = u8;
type Con = u16;

pub enum Opcode {
    Exit,
    Load(Reg, Con),
    Add(Reg, Reg, Reg),
    Sub(Reg, Reg, Reg),
    Print(Reg)
}

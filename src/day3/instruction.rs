use super::mul_instruction::MulInstruction;

pub(super) enum Instruction {
    Mul(MulInstruction),
    Do,
    Dont,
}
impl Instruction {
    pub fn is_mul(&self) -> bool {
        match self {
            Self::Mul(_) => true,
            _ => false,
        }
    }
    pub fn unwrap_mul(self) -> MulInstruction {
        match self {
            Self::Mul(mul) => mul,
            _ => panic!("This function should only be called in mul enums"),
        }
    }
}


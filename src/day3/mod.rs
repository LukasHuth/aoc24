use crate::{build_run, build_test, utilities::datatypes::num_wrapper::NumWrapper};

fn part1() -> u32 {
    let mut result = NumWrapper::new(0u32);
    let instructions = load_instructions()
        .into_iter()
        .filter(Instruction::is_mul)
        .map(Instruction::unwrap_mul);
    for instruction in instructions {
        result += instruction;
    }
    *result
}

fn part2() -> u32 {
    let mut result = NumWrapper::new(0);
    let instructions = load_instructions();
    let mut dont = false;
    for instruction in instructions {
        match instruction {
            Instruction::Do => dont = false,
            Instruction::Dont => dont = true,
            Instruction::Mul(mul) => {
                if !dont {
                    result += mul;
                }
            }
        }
    }
    *result
}

mod mul_instruction;
use mul_instruction::*;
mod instruction;
use instruction::*;

fn load_instructions() -> Vec<Instruction> {
    let data = include_str!("input.txt");
    let mut pointer = 0;
    let mut instructions = Vec::new();
    while (data.len() - pointer) > 8 {
        let size = (data.len() - pointer).clamp(8, 12);
        let sample = &data[pointer..pointer + size];
        if let Ok((instruction, used_length)) = MulInstruction::new(sample) {
            instructions.push(Instruction::Mul(instruction));
            pointer += used_length;
        } else if &sample[0..7] == "don't()" {
            instructions.push(Instruction::Dont);
            pointer += 7;
        } else if &sample[0..4] == "do()" {
            instructions.push(Instruction::Do);
            pointer += 4;
        } else {
            pointer += 1;
        }
    }
    instructions
}
build_run!(3, part1, part2);
build_test!(part1: 173785482, part2: 83158140);

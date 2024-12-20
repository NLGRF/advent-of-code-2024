use regex::{Captures, RegexBuilder};
use std::fs;

enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

pub fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read the file.");

    let re = RegexBuilder::new(r"mul\((?<x>\d+),(?<y>\d+)\)|do\(\)|don't\(\)")
        .build()
        .unwrap();

    let instructions = re
        .captures_iter(&input)
        .map(parse_instruction)
        .collect::<Vec<Instruction>>();

    let part2 = do_instructions(&instructions, false);

    println!("Part 2: {}", part2);
}

fn parse_instruction(instruction: Captures) -> Instruction {
    match instruction.get(0).map(|m| m.as_str()) {
        Some("do()") => Instruction::Do,
        Some("don't()") => Instruction::Dont,
        Some(_) => {
            let x = instruction.name("x").expect("Missing x in mul");
            let y = instruction.name("y").expect("Missing y in mul");
            Instruction::Mul(
                x.as_str().parse::<i32>().expect("Invalid x value"),
                y.as_str().parse::<i32>().expect("Invalid y value"),
            )
        }
        None => panic!("Invalid instruction"),
    }
}

fn do_instructions(instructions: &[Instruction], ignore_dos: bool) -> i32 {
    let mut active = true;
    let mut sum = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Mul(x, y) => {
                if active {
                    sum += x * y;
                }
            }
            Instruction::Do => {
                if !ignore_dos {
                    active = true;
                }
            }
            Instruction::Dont => {
                if !ignore_dos {
                    active = false;
                }
            }
        }
    }
    sum
}

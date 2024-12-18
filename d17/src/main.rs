use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut register_mode = true;
    let mut registers = Vec::new();
    let mut instructions = Vec::new();

    contents.lines().for_each(|line| {
        if line.is_empty() {
            register_mode = false;
            return;
        }
        if register_mode {
            registers.push(line.split(": ").nth(1).unwrap().parse::<u128>().unwrap())
        } else {
            line.split(": ").nth(1).unwrap().split(",").for_each(|num| {
                instructions.push(num.parse::<i8>().unwrap());
            });
        }
    });

    // Join output to string, separated by commas
    let output_str = run(&mut registers, &instructions).iter().map(|num| num.to_string()).collect::<Vec<String>>().join(",");

    println!("Output: {}", output_str);

    // let register_a = solve(&instructions);

    // println!("Register A: {}", register_a);

    Ok(())
}

// const COMB_A: i8 = 4;
// const COMB_B: i8 = 5;
// const COMB_C: i8 = 6;
// const MAX_LOOP: u128 = 9192;

fn run(registers: &mut Vec<u128>, instructions: &Vec<i8>) -> Vec<i8> {
    let mut outputs = Vec::new();

    let mut pc = 0usize;

    while pc < instructions.len() - 1 {
        let op_code = instructions[pc];
        let operand = instructions[pc + 1];

        match op_code {
            0 => {
                let exp = get_combo_operand(operand, &registers);
                if exp > 128 {
                    registers[0] = 0;
                }
                registers[0] /= 2u128.pow(exp as u32);
            }
            1 => {
                registers[1] ^= operand as u128;
            }
            2 => {
                registers[1] = get_combo_operand(operand, &registers) % 8;
            }
            3 => {
                if registers[0] != 0 {
                    pc = operand as usize;
                    continue;
                }
            }
            4 => {
                registers[1] ^= registers[2];
            }
            5 => {
                outputs.push((get_combo_operand(operand, &registers) % 8) as i8);
            }
            6 => {
                let exp = get_combo_operand(operand, &registers);
                if exp > 128 {
                    registers[1] = 0;
                }
                registers[1] = registers[0] / 2u128.pow(exp as u32);
            }
            7 => {
                let exp = get_combo_operand(operand, &registers);
                if exp > 128 {
                    registers[2] = 0;
                }
                registers[2] = registers[0] / 2u128.pow(exp as u32);
            }
            _ => panic!("Invalid opcode: {}", op_code)
        }

        pc += 2;
    }

    outputs
}

fn get_combo_operand(op_no: i8, registers: &Vec<u128>) -> u128 {
    match op_no {
        0 | 1 | 2 | 3 => op_no as u128,
        4 | 5 | 6 => registers[(op_no - 4) as usize],
        _ => panic!("Invalid combo operand: {}", op_no)
    }
}

// fn solve(instructions: &Vec<i8>) -> u128 {
//     // A must be 0, B and C can be anything
//     let mut registers = vec![0, 0, 0];
//     let output_index = instructions.len() as i32 - 1;
//     // Start from the last jump instruction
//     let mut instruction_index = -1;
//     for i in (0..output_index + 1).rev().filter(|i| i % 2 == 0) {
//         if instructions[i as usize] == 3 {
//             instruction_index = i;
//             break;
//         }
//     }
//
//     reverse(&mut registers, instructions, instruction_index, output_index).unwrap()
// }
//
// fn reverse(registers: &mut Vec<u128>, instructions: &Vec<i8>, instruction_index: i32, output_index: i32) -> Option<u128> {
//     if instruction_index < 0 {
//         return reverse(registers, instructions, instructions.len() as i32 - 2, output_index);
//     }
//
//     // println!("Registers: {:?}, Instruction Index: {}, Output: {}", registers, instruction_index, instructions[output_index as usize]);
//
//     let op_code = instructions[instruction_index as usize];
//     let operand = instructions[instruction_index as usize + 1];
//
//     match op_code {
//         0 => {
//             if operand == COMB_A {
//                 registers[0] = 0;
//                 return reverse(registers, instructions, instruction_index - 2, output_index);
//             }
//
//             let denominator = 2u128.pow(get_combo_operand(operand, &registers));
//             let base_a = registers[0] * denominator;
//             for i in 0..denominator {
//                 registers[0] = base_a + i;
//                 let res = reverse(registers, instructions, instruction_index - 2, output_index);
//                 if res.is_some() {
//                     return res;
//                 }
//             }
//
//             None
//         }
//         1 => {
//             registers[1] ^= operand as u128;
//             reverse(registers, instructions, instruction_index - 2, output_index)
//         }
//         2 => {
//             if operand == COMB_B {
//                 // B mod 8 = B
//                 if registers[1] >= 8 {
//                     return None;
//                 }
//                 let mut counter = registers[1];
//                 loop {
//                     registers[1] = counter;
//                     let res = reverse(registers, instructions, instruction_index - 2, output_index);
//                     if res.is_some() {
//                         return res;
//                     }
//                     counter += 8;
//                 }
//             } else {
//                 let expected = get_combo_operand(operand, &registers) % 8;
//                 if registers[1] != expected {
//                     return None;
//                 }
//             }
//
//             // B can be anything before the instruction
//             let mut counter = 0;
//             loop {
//                 registers[1] = counter;
//                 let res = reverse(registers, instructions, instruction_index - 2, output_index);
//                 if res.is_some() {
//                     return res;
//                 }
//                 counter += 1;
//             }
//         }
//         3 => {
//             // Jump is always at the end
//             reverse(registers, instructions, instruction_index - 2, output_index)
//         }
//         4 => {
//             registers[1] ^= registers[2];
//             reverse(registers, instructions, instruction_index - 2, output_index)
//         }
//         5 => {
//             if output_index < 0 {
//                 if registers[1] == 0 && registers[2] == 0 {
//                     return Some(registers[0]);
//                 }
//
//                 return None;
//             }
//
//             if (get_combo_operand(operand, &registers) % 8) as i8 != instructions[output_index as usize] {
//                 return None;
//             }
//
//             reverse(registers, instructions, instruction_index - 2, output_index - 1)
//         }
//         6 => {
//             let exp = get_combo_operand(operand, &registers);
//             if exp > 32 {
//                 if registers[1] != 0 {
//                     return None;
//                 }
//             } else {
//                 let expected = registers[0] / 2u128.pow(exp);
//                 if registers[1] != expected {
//                     return None;
//                 }
//             }
//
//             // B can be anything before the instruction
//             let mut counter = 0;
//             let curr_b = registers[1];
//             let counter_max = MAX_LOOP;
//             while counter <= counter_max {
//                 if operand == COMB_B && registers[0] / 2u128.pow(counter) != curr_b {
//                     counter += 1;
//                     continue;
//                 }
//                 registers[1] = counter;
//                 let res = reverse(registers, instructions, instruction_index - 2, output_index);
//                 if res.is_some() {
//                     return res;
//                 }
//                 counter += 1;
//             }
//
//             None
//         }
//         7 => {
//             let exp = get_combo_operand(operand, &registers);
//             if exp > 32 {
//                 if registers[2] != 0 {
//                     return None;
//                 }
//             } else {
//                 let expected = registers[0] / 2u128.pow(exp);
//                 if registers[2] != expected {
//                     return None;
//                 }
//             }
//
//             // C can be anything before the instruction
//             let mut counter = 0;
//             let curr_c = registers[2];
//             let counter_max = MAX_LOOP;
//             while counter <= counter_max {
//                 if operand == COMB_C && registers[0] / 2u128.pow(counter) != curr_c {
//                     counter += 1;
//                     continue;
//                 }
//                 registers[2] = counter;
//                 let res = reverse(registers, instructions, instruction_index - 2, output_index);
//                 if res.is_some() {
//                     return res;
//                 }
//                 counter += 1;
//             }
//
//             None
//         }
//         _ => panic!("Invalid opcode: {}", op_code)
//     }
// }
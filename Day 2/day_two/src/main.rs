use std::fs;
use std::string::String;

fn main() {
    let filename = "src/input.txt";
    let contents: String = fs::read_to_string(filename).expect("Guess the filename is incorrect");

    let mut instructions: Vec<i32> = contents
        .split(",")
        .map(|input| input.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut program_counter: u32 = 0;
    let mut position_zero: u32 = 0;
    let mut found_pos_zero: bool = false;

    while instructions[program_counter as usize] != 99 {
        let current_num: i32 = instructions[program_counter as usize];
        program_counter += match current_num {
            1 => {
                opcode_add(&mut instructions, program_counter);
                4
            }
            2 => {
                opcode_multiply(&mut instructions, program_counter);
                4
            }
            _ => {
                if found_pos_zero {
                    unreachable!("CODE FOUND ERROR");
                }
                1
            }
        };

        if current_num + 1 == program_counter as i32 {
            position_zero = program_counter;
            found_pos_zero = true;
        }
    }

    println!("{}", instructions[position_zero as usize]);
}

fn opcode_add(program: &mut Vec<i32>, program_index: u32) {
    let variable_one_index: usize = program[(program_index + 1) as usize] as usize;
    let variable_two_index: usize = program[(program_index + 2) as usize] as usize;
    let storage_index: usize = program[(program_index + 3) as usize] as usize;

    program[storage_index] = program[variable_one_index] + program[variable_two_index];
}

fn opcode_multiply(program: &mut Vec<i32>, program_index: u32) {
    let variable_one_index: usize = program[(program_index + 1) as usize] as usize;
    let variable_two_index: usize = program[(program_index + 2) as usize] as usize;
    let storage_index: usize = program[(program_index + 3) as usize] as usize;

    program[storage_index] = program[variable_one_index] * program[variable_two_index];
}

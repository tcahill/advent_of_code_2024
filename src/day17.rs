use std::collections::VecDeque;

use itertools::Itertools;

fn parse_input(input: &str) -> (Vec<u64>, Vec<u8>) {
    let mut lines = input.lines();

    let registers = lines.by_ref().take(3).map(|line| {
        let (_, value) = line.split_once(':').unwrap();
        value.trim().parse().unwrap()
    }).collect();

    let (_, program_str) = lines.skip(1).next().unwrap().split_once(':').unwrap();
    let program = program_str.split(',').map(|i| i.trim().parse().unwrap()).collect();

    (registers, program)
}

fn combo_operand(value: u8, registers: &Vec<u64>) -> u64 {
    match value {
        0..4 => value as u64,
        4..7 => registers[value as usize - 4],
        _ => panic!("unexpected combo operand value"),
    }
}

fn run_program(program: &Vec<u8>, registers: &mut Vec<u64>) -> Vec<u8> {
    let mut output = Vec::new();

    let mut program_counter = 0;
    let mut jumped = false;

    while program_counter < program.len() {
        match program[program_counter] {
            0 => {
              divide(program[program_counter + 1], registers, 0);
            },
            1 => {
                println!("bxl {}", program[program_counter + 1]);
                registers[1] = registers[1] ^ program[program_counter + 1] as u64;
            },
            2 => {
                println!("bst {}", combo_operand(program[program_counter + 1], registers));
                registers[1] = combo_operand(program[program_counter + 1], registers) % 8;
            }
            3 => {
                println!("jnz");
                if registers[0] != 0 {
                    program_counter = program[program_counter + 1] as usize;
                    jumped = true;
                }
            },
            4 => {
                println!("bxc");
                registers[1] = registers[1] ^ registers[2];
            },
            5 => {
                println!("out {}", combo_operand(program[program_counter + 1], registers) % 8);
                output.push((combo_operand(program[program_counter + 1], registers) % 8) as u8);
            }
            6 => {
                divide(program[program_counter + 1], registers, 1);
            },
            7 => {
                divide(program[program_counter + 1], registers, 2);
            },
            _ => panic!("unexpected instruction value"),
        }

        if jumped == true {
            jumped = false;
        } else {
            program_counter += 2;
        }

        println!("registers: {:?}\n", registers);
    }

    output
}

fn run_once(program: &Vec<u8>, registers: &mut Vec<u64>) -> u8 {
    let mut output = 0;

    let mut program_counter = 0;

    while program_counter < program.len() {
        match program[program_counter] {
            0 => {
              divide(program[program_counter + 1], registers, 0);
            },
            1 => {
                registers[1] = registers[1] ^ program[program_counter + 1] as u64;
            },
            2 => {
                registers[1] = combo_operand(program[program_counter + 1], registers) % 8;
            }
            3 => {
                return output;
            },
            4 => {
                registers[1] = registers[1] ^ registers[2];
            },
            5 => {
                output = (combo_operand(program[program_counter + 1], registers) % 8) as u8;
            }
            6 => {
                divide(program[program_counter + 1], registers, 1);
            },
            7 => {
                divide(program[program_counter + 1], registers, 2);
            },
            _ => panic!("unexpected instruction value"),
        }

        program_counter += 2;
    }

    output
}


fn divide(operand: u8, registers: &mut Vec<u64>, result_index: usize) {
    let value = combo_operand(operand, registers);

    registers[result_index] = registers[0] / 2_u64.pow(value as u32);
}

pub fn part1(input: &str) -> u64 {
    let (mut registers, program) = parse_input(input);
    let output = run_program(&program, &mut registers);
    println!("{}", output.iter().map(|b| b.to_string() ).join(","));
    0
}

pub fn part2(input: &str) -> u64 {
    let (_, program) = parse_input(input);
    let mut registers = vec![0,0,0];

    let mut queue = VecDeque::from([(0, program.len() - 1)]);
    while let Some((acc, remaining)) = queue.pop_back() {
        for i in 0..8 {
            registers[0] = acc << 3 | i;
            let output = run_once(&program, &mut registers);
            if output == program[remaining] {
                if remaining == 0 {
                    return acc << 3 | i;
                }
                queue.push_front((acc << 3 | i, remaining - 1));
            }
        }
    }

    0
}

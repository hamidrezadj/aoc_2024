use std::ops::{BitXor, ControlFlow};

struct State {
    a: u64,
    b: u64,
    c: u64,
    ///This represents the Instruction Pointer.
    ip: usize,
}

fn main() {
    let mut input = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"));
    let _a: u64 = input
        .next()
        .expect("Unexpected pattern: No first line")
        .split_once("Register A: ")
        .expect("Unexpected pattern: Invalid first line")
        .1
        .parse::<u64>()
        .expect("Unexpected Pattern: Could not parse a u64 number at first line");
    let b: u64 = input
        .next()
        .expect("Unexpected pattern: No second line")
        .split_once("Register B: ")
        .expect("Unexpected pattern: Invalid second line")
        .1
        .parse::<u64>()
        .expect("Unexpected Pattern: Could not parse a u64 number at second line");
    let c: u64 = input
        .next()
        .expect("Unexpected pattern: No third line")
        .split_once("Register C: ")
        .expect("Unexpected pattern: Invalid third line")
        .1
        .parse::<u64>()
        .expect("Unexpected Pattern: Could not parse a u64 number at third line");
    input
        .next()
        .filter(|line| line.is_empty())
        .expect("Unexpected pattern: No empty fourth line");
    let instructions: Vec<u64> = input
        .next()
        .expect("Unexpected pattern: No fifth line")
        .split_once("Program: ")
        .expect("Unexpected pattern: Invalid fifth line")
        .1
        .split(',')
        .map(|split| {
            split
                .parse::<u8>()
                .expect("Unexpected pattern: Instructions could not be parsed into an u8")
                as u64
        })
        .collect();
    if input.count() > 0 {
        panic!("Unexpected pattern: more than five lines")
    }

    let mut args = std::env::args().skip(1);
    match args.next().unwrap_or_default().as_str() {
        "brute" => {
            let output = (0..)
                .map(|a| State { a, b, c, ip: 0 })
                .map(|initial_state| cpu_factory(initial_state, &instructions))
                .enumerate()
                .try_for_each(|(a, cpu)| {
                    let instructions = instructions.iter().copied().map(|i| i as u8);
                    if cpu.eq(instructions) {
                        ControlFlow::Break(a)
                    } else {
                        ControlFlow::Continue(())
                    }
                })
                .break_value()
                .unwrap();
            println!("{}", output);
        }
        "print" => {
            for a in 0.. {
                let initial_state = State { a, b, c, ip: 0 };
                let cpu = cpu_factory(initial_state, &instructions);
                let output = cpu.collect::<Vec<u8>>();
                println!("{:?}", output);
            }
        }
        "solve" => {
            let factor = args
                .next()
                .expect("Factor argument not passed")
                .parse::<u64>()
                .expect("Could not parse factor argument to an unsigned 64 bit integer");
            if instructions.len() > 64 {
                eprintln!("Cannot use this solution for more than 63 instruction.");
                eprintln!("Instructions length is the exponent of factor,");
                eprintln!("and the result must fit in 64 bits");
                panic!("Instructions list too long");
            }
            let mut a_in_base_factor = vec![0u64; instructions.len() + 1];
            let output = loop {
                let a = a_in_base_factor
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(exponent, a_digit)| a_digit * factor.pow(exponent as u32))
                    .sum::<u64>();
                let mut desired_outputs: Vec<Option<u8>> = instructions
                    .iter()
                    .copied()
                    .flat_map(TryInto::try_into)
                    .map(Option::Some)
                    .collect();
                let mut actual_outputs = cpu_factory(State { a, b, c, ip: 0 }, &instructions)
                    .map(Option::Some)
                    .collect::<Vec<Option<u8>>>();
                desired_outputs.extend(vec![None; 1]);
                actual_outputs.extend(vec![
                    None;
                    (instructions.len() + 1)
                        .saturating_sub(actual_outputs.len())
                ]);
                let (reverse_disparity_index, (_actual, _desired)) = match actual_outputs
                    .into_iter()
                    .zip(desired_outputs)
                    .rev()
                    .enumerate()
                    .find(|(_idx, (actual, desired))| actual != desired)
                {
                    Some(disparity) => disparity,
                    None => break a,
                };
                if reverse_disparity_index == 0 {
                    eprintln!("Could not find the answer.");
                    eprintln!("Output's length exceeded the instruction list's length.");
                    panic!("Could not find the answer");
                }
                a_in_base_factor[reverse_disparity_index] += 1;
                if a_in_base_factor[reverse_disparity_index] >= factor {
                    a_in_base_factor[reverse_disparity_index] = 0;
                    a_in_base_factor[reverse_disparity_index - 1] += 1;
                }
            };
            println!("{}", output);
        }
        _ => {
            eprintln!("Invalid arguments.");
            eprintln!("Usage:");
            eprintln!("p2 brute < input");
            eprintln!("p2 print < input");
            eprintln!("p2 solve (factor e.g. 8) < input");
        }
    }
}

fn cpu_factory(initial_state: State, instructions: &[u64]) -> impl Iterator<Item = u8> + '_ {
    (0..)
        .scan(initial_state, |state, _| {
            if state.ip >= instructions.len() {
                return None;
            }
            // instructions.len() == 0 is checked above,
            // so it doesn't cause any problems for the check below.
            if state.ip >= instructions.len() - 1 {
                panic!("No operand for instruction");
            }
            let instruction = instructions[state.ip];
            let operand = instructions[state.ip + 1];
            let mut output = None;
            match instruction {
                0 => adv(operand, state),
                1 => bxl(operand, state),
                2 => bst(operand, state),
                3 => jnz(operand, state),
                4 => bxc(state),
                5 => output = Some(out(operand, state)),
                6 => bdv(operand, state),
                7 => cdv(operand, state),
                _ => panic!("Invalid instruction"),
            }
            Some(output)
        })
        .flatten()
}

fn cdv(operand: u64, state: &mut State) {
    let operand = combo_operand(operand, state);
    if operand >= 64 {
        panic!("Overflow in cdv denominator: Exponent of 2 bigger than 63");
    }
    let c = state.a / 2u64.pow(operand as u32);
    let ip = state.ip + 2;
    *state = State { c, ip, ..*state };
}

fn bdv(operand: u64, state: &mut State) {
    let operand = combo_operand(operand, state);
    if operand >= 64 {
        panic!("Overflow in cdv denominator: Exponent of 2 bigger than 63");
    }
    let b = state.a / 2u64.pow(operand as u32);
    let ip = state.ip + 2;
    *state = State { b, ip, ..*state };
}

fn out(operand: u64, state: &mut State) -> u8 {
    let operand = combo_operand(operand, state);
    let ip = state.ip + 2;
    *state = State { ip, ..*state };
    (operand % 8) as u8
}

fn bxc(state: &mut State) {
    let b = state.b.bitxor(state.c);
    let ip = state.ip + 2;
    *state = State { b, ip, ..*state };
}

fn jnz(operand: u64, state: &mut State) {
    let ip = if state.a != 0 {
        // This cast doesn't fail on any modern computer
        // since literal operands fit into u8 and that
        // fits in even 8 bit computers' usize type.
        operand as usize
    } else {
        state.ip + 2
    };
    *state = State { ip, ..*state };
}

fn bst(operand: u64, state: &mut State) {
    let operand = combo_operand(operand, state);
    let b = operand % 8;
    let ip = state.ip + 2;
    *state = State { b, ip, ..*state };
}

fn bxl(operand: u64, state: &mut State) {
    let b = state.b.bitxor(operand);
    let ip = state.ip + 2;
    *state = State { b, ip, ..*state };
}

fn adv(operand: u64, state: &mut State) {
    let operand = combo_operand(operand, state);
    if operand >= 64 {
        panic!("Overflow in cdv denominator: Exponent of 2 bigger than 63");
    }
    let a = state.a / 2u64.pow(operand as u32);
    let ip = state.ip + 2;
    *state = State { a, ip, ..*state };
}

fn combo_operand(o: u64, s: &State) -> u64 {
    match o {
        o @ 0..=3 => o,
        4 => s.a,
        5 => s.b,
        6 => s.c,
        _ => panic!("Invalid combo operand"),
    }
}

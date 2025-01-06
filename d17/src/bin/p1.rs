use std::{fmt::Write, ops::BitXor};

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
    let a: u64 = input
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

    let mut state = State { a, b, c, ip: 0 };
    let mut output = Vec::new();
    loop {
        if state.ip >= instructions.len() {
            break;
        }
        // instructions.len() == 0 is checked above,
        // so it doesn't cause any problems for the check below.
        if state.ip >= instructions.len() - 1 {
            panic!("No operand for instruction");
        }
        let instruction = instructions[state.ip];
        let operand = instructions[state.ip + 1];
        match instruction {
            0 => adv(operand, &mut state),
            1 => bxl(operand, &mut state),
            2 => bst(operand, &mut state),
            3 => jnz(operand, &mut state),
            4 => bxc(&mut state),
            5 => output.push(out(operand, &mut state)),
            6 => bdv(operand, &mut state),
            7 => cdv(operand, &mut state),
            _ => panic!("Invalid instruction"),
        }
    }

    let output = output
        .iter()
        .fold(String::with_capacity(output.len() * 2), |mut acc, n| {
            write!(acc, "{},", n).unwrap();
            acc
        })
        .rsplit_once(',')
        .map(|(l, _r)| l)
        .unwrap_or("")
        .to_owned();
    println!("{}", output);
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

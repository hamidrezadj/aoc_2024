use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    ops::Not,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Gate {
    And,
    Or,
    Xor,
}

type Wire = (u8, u8, u8);
type Inputs = [Wire; 2];
type Circuit = HashMap<Wire, (Gate, Inputs)>;

type RCircuit = HashMap<(Gate, Inputs), Wire>;
type SymbolicWire = Wire;
type WannabeWire = Wire;
type RealWire = Wire;
type Priority = u64;
type Depth = u8;
type Corrections = Vec<(SymbolicWire, WannabeWire, RealWire)>;
type CorrectionsQueue = BinaryHeap<(Priority, Corrections)>;

fn main() {
    let mut _input_wires = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .take_while(|line| line.is_empty().not())
        .count();
    let circuit: Circuit = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            let (lhs, rhs) = line
                .split_once(" -> ")
                .expect("no ' -> ' pattern in second section of input");
            let output = encoded(rhs);
            let lhs = lhs.split_whitespace().collect::<Vec<_>>();
            let gate = match lhs[1] {
                "AND" => Gate::And,
                "OR" => Gate::Or,
                "XOR" => Gate::Xor,
                _ => panic!("Invalid gate type"),
            };
            let input_1 = encoded(lhs[0]);
            let input_2 = encoded(lhs[2]);
            let mut inputs = [input_1, input_2];
            inputs.sort();
            (output, (gate, inputs))
        })
        .collect();

    let biggest_z = circuit
        .keys()
        .filter(|wire| starts_with(**wire, b'z'))
        .max()
        .copied()
        .expect("No output that starts with z");
    let bits = decoded(biggest_z)[1..3]
        .parse::<Depth>()
        .expect("z output with invalid numeric suffix")
        + 1;
    assert!(bits <= 64, "Too big of an input and output bit count");

    let all_wires: Vec<Wire> = circuit.keys().cloned().collect();
    assert!(all_wires.is_empty().not(), "Empty input");
    let scs: Vec<Inputs> = all_wires
        .iter()
        .copied()
        .map(|wire| circuit.get(&wire).unwrap())
        .filter(|(gate, [i0, i1])| {
            gate == &Gate::Xor
                && starts_with(*i0, b'x').not()
                && starts_with(*i0, b'y').not()
                && starts_with(*i1, b'x').not()
                && starts_with(*i1, b'y').not()
        })
        .map(|(_gate, inputs)| inputs.to_owned())
        .collect();
    let abs: Vec<Inputs> = all_wires
        .iter()
        .copied()
        .map(|wire| circuit.get(&wire).unwrap())
        .filter(|(gate, _inputs)| gate == &Gate::Or)
        .map(|(_gate, inputs)| inputs.to_owned())
        .collect();
    let r_circuit: RCircuit = circuit
        .clone()
        .into_iter()
        .map(|(output, (gate, inputs))| ((gate, inputs), output))
        .collect();

    let z00_symbolic = encoded("z00");
    let z00_wannabe = r_circuit
        .get(&(Gate::Xor, encoded_sorted_inputs("x00", "y00")))
        .copied()
        .unwrap();
    let z00_real = encoded("z00");

    let mut corrections_queue: CorrectionsQueue = Default::default();
    corrections_queue.push((0, vec![(z00_symbolic, z00_wannabe, z00_real)]));

    while let Some((_priority, corrections)) = corrections_queue.pop() {
        let &(last_symbol, _, _) = corrections.last().unwrap();
        let last_depth = depth_from_symbolic_encoded_wire(last_symbol);
        let used_input_pairs: Vec<Inputs> = corrections
            .iter()
            // Skip first z
            .skip(1)
            .enumerate()
            // Skip rest of the zs
            .filter(|(index, _)| index % 5 != 4)
            .map(|(_, correction)| correction)
            .map(|(_, _, real)| real)
            .copied()
            .collect::<Vec<RealWire>>()
            // Group input pairs
            .windows(2)
            .step_by(2)
            .map(|window| sorted_inputs(window[0], window[1]))
            .collect();
        let last_stage = last_symbol.0;
        match last_stage {
            b'z' if last_depth < bits - 2 => {
                let depth = last_depth + 1;
                let stage_priority_increment = 0;

                let sn_wannabe = {
                    let xn_real = encoded(&format!("x{:02}", depth));
                    let yn_real = encoded(&format!("y{:02}", depth));
                    r_circuit
                        .get(&(Gate::Xor, sorted_inputs(xn_real, yn_real)))
                        .copied()
                        .unwrap()
                };
                if corrections
                    .iter()
                    .any(|(_, wannabe, _)| *wannabe == sn_wannabe)
                {
                    // Prevents loops
                    continue;
                }

                let cn_1_wannabe = if last_depth == 0 {
                    r_circuit
                        .get(&(Gate::And, encoded_sorted_inputs("x00", "y00")))
                        .copied()
                        .unwrap()
                } else {
                    let an_1_symbol = encoded(&format!("a{:02}", last_depth));
                    let bn_1_symbol = encoded(&format!("b{:02}", last_depth));
                    let an_1_real = corrections
                        .iter()
                        .find(|(symbol, _, _)| *symbol == an_1_symbol)
                        .map(|(_, _, real)| real)
                        .copied()
                        .unwrap();
                    let bn_1_real = corrections
                        .iter()
                        .find(|(symbol, _, _)| *symbol == bn_1_symbol)
                        .map(|(_, _, real)| real)
                        .copied()
                        .unwrap();
                    r_circuit
                        .get(&(Gate::Or, sorted_inputs(an_1_real, bn_1_real)))
                        .copied()
                        .unwrap()
                };
                if corrections
                    .iter()
                    .any(|(_, wannabe, _)| *wannabe == cn_1_wannabe)
                {
                    continue;
                }

                let sn_symbol = encoded(&format!("s{:02}", depth));
                let cn_1_symbol = encoded(&format!("c{:02}", last_depth));
                let interesting_sc_pairs = scs
                    .iter()
                    .filter(|sc_pair| used_input_pairs.contains(*sc_pair).not())
                    .cloned();
                for [i0, i1] in interesting_sc_pairs {
                    swap_two_wires_and_push_to_queue(
                        &mut corrections_queue,
                        &corrections,
                        depth as Priority * 10 + stage_priority_increment,
                        (sn_symbol, sn_wannabe, i0),
                        (cn_1_symbol, cn_1_wannabe, i1),
                    );
                    swap_two_wires_and_push_to_queue(
                        &mut corrections_queue,
                        &corrections,
                        depth as Priority * 10 + stage_priority_increment,
                        (sn_symbol, sn_wannabe, i1),
                        (cn_1_symbol, cn_1_wannabe, i0),
                    );
                }
            }
            b'c' => {
                let depth = last_depth + 1;
                let stage_priority_increment = 1;

                let an_wannabe = {
                    let xn_real = encoded(&format!("x{:02}", depth));
                    let yn_real = encoded(&format!("y{:02}", depth));
                    r_circuit
                        .get(&(Gate::And, sorted_inputs(xn_real, yn_real)))
                        .copied()
                        .unwrap()
                };
                if corrections
                    .iter()
                    .any(|(_, wannabe, _)| *wannabe == an_wannabe)
                {
                    continue;
                }

                let bn_wannabe = {
                    let sn_symbol = encoded(&format!("s{:02}", depth));
                    let cn_1_symbol = encoded(&format!("c{:02}", depth - 1));
                    let sn_real = corrections
                        .iter()
                        .find(|(symbol, _, _)| *symbol == sn_symbol)
                        .map(|(_, _, real)| real)
                        .copied()
                        .unwrap();
                    let cn_1_real = corrections
                        .iter()
                        .find(|(symbol, _, _)| *symbol == cn_1_symbol)
                        .map(|(_, _, real)| real)
                        .copied()
                        .unwrap();
                    r_circuit
                        .get(&(Gate::And, sorted_inputs(sn_real, cn_1_real)))
                        .copied()
                        .unwrap()
                };
                if corrections
                    .iter()
                    .any(|(_, wannabe, _)| *wannabe == bn_wannabe)
                {
                    continue;
                }

                let an_symbol = encoded(&format!("a{:02}", depth));
                let bn_symbol = encoded(&format!("b{:02}", depth));
                let interesting_ab_pairs = abs
                    .iter()
                    .filter(|ab_pair| used_input_pairs.contains(*ab_pair).not())
                    .cloned();
                for [i0, i1] in interesting_ab_pairs {
                    swap_two_wires_and_push_to_queue(
                        &mut corrections_queue,
                        &corrections,
                        depth as Priority * 10 + stage_priority_increment,
                        (an_symbol, an_wannabe, i0),
                        (bn_symbol, bn_wannabe, i1),
                    );
                    swap_two_wires_and_push_to_queue(
                        &mut corrections_queue,
                        &corrections,
                        depth as Priority * 10 + stage_priority_increment,
                        (an_symbol, an_wannabe, i1),
                        (bn_symbol, bn_wannabe, i0),
                    );
                }
            }
            b'b' => {
                let depth = last_depth;
                let stage_priority_increment = 2;

                let zn_symbol = encoded(&format!("z{:02}", depth));
                let zn_wannabe = {
                    let sn_symbol = encoded(&format!("s{:02}", depth));
                    let cn_1_symbol = encoded(&format!("c{:02}", depth - 1));
                    let sn_real = corrections
                        .iter()
                        .find(|(symbol, _, _)| *symbol == sn_symbol)
                        .map(|(_, _, real)| real)
                        .copied()
                        .unwrap();
                    let cn_1_real = corrections
                        .iter()
                        .find(|(symbol, _, _)| *symbol == cn_1_symbol)
                        .map(|(_, _, real)| real)
                        .copied()
                        .unwrap();
                    r_circuit
                        .get(&(Gate::Xor, sorted_inputs(sn_real, cn_1_real)))
                        .copied()
                        .unwrap()
                };
                let zn_real = zn_symbol;
                if corrections
                    .iter()
                    .any(|(_, wannabe, _)| *wannabe == zn_wannabe)
                {
                    // Loop detected
                    continue;
                }
                if corrections
                    .iter()
                    .find(|(_, _, real)| *real == zn_wannabe)
                    .filter(|(_, wannabe, _)| *wannabe != zn_real)
                    .is_some()
                {
                    // Conflicting transfer found
                    continue;
                }
                let good_fit_priority_increment = match zn_wannabe == zn_real {
                    true => 2000,
                    false => 1000,
                };
                let new_priority =
                    depth as Priority * 10 + stage_priority_increment + good_fit_priority_increment;
                let mut corrections = corrections;
                corrections.push((zn_symbol, zn_wannabe, zn_real));
                corrections_queue.push((new_priority, corrections));
            }
            b'z' if last_depth == bits - 2 => {
                let depth = last_depth + 1;
                let zn_symbol = encoded(&format!("z{:02}", depth));
                let zn_wannabe = {
                    let an_1_symbol = encoded(&format!("a{:02}", depth - 1));
                    let bn_1_symbol = encoded(&format!("b{:02}", depth - 1));
                    let an_1_real = corrections
                        .iter()
                        .find(|(symbol, _, _)| *symbol == an_1_symbol)
                        .map(|(_, _, real)| real)
                        .copied()
                        .unwrap();
                    let bn_1_real = corrections
                        .iter()
                        .find(|(symbol, _, _)| *symbol == bn_1_symbol)
                        .map(|(_, _, real)| real)
                        .copied()
                        .unwrap();
                    r_circuit
                        .get(&(Gate::Or, sorted_inputs(an_1_real, bn_1_real)))
                        .copied()
                        .unwrap()
                };
                let zn_real = zn_symbol;
                if corrections
                    .iter()
                    .any(|(_, wannabe, _)| *wannabe == zn_wannabe)
                {
                    // Loop detected
                    continue;
                }
                if corrections
                    .iter()
                    .find(|(_, _, real)| *real == zn_wannabe)
                    .filter(|(_, wannabe, _)| *wannabe != zn_real)
                    .is_some()
                {
                    // Conflicting transfer found
                    continue;
                }
                let mut corrections = corrections;
                corrections.push((zn_symbol, zn_wannabe, zn_real));

                if corrections
                    .iter()
                    .filter(|(_, lhs, rhs)| lhs != rhs)
                    .count()
                    == 4 * 2
                {
                    let output = corrections
                        .into_iter()
                        .filter(|(_, lhs, rhs)| lhs != rhs)
                        .map(|(_, _, real)| real)
                        .collect::<HashSet<Wire>>()
                        .into_iter()
                        .map(decoded)
                        .collect::<BinaryHeap<String>>()
                        .into_sorted_vec()
                        .join(",");
                    println!("{}", output);
                    return;
                }
            }
            _ => unreachable!(),
        }
    }
}

fn swap_two_wires_and_push_to_queue(
    corrections_queue: &mut CorrectionsQueue,
    corrections: &Corrections,
    base_priority: Priority,
    (sym0, lhs0, rhs0): (SymbolicWire, WannabeWire, RealWire),
    (sym1, lhs1, rhs1): (SymbolicWire, WannabeWire, RealWire),
) {
    let priority_increment = match (lhs0 == rhs0, lhs1 == rhs1) {
        (true, true) => 2000,
        (true, false) => 1000,
        (false, true) => 1000,
        (false, false) => 0,
    };
    if corrections
        .iter()
        .find(|(_, _, real)| *real == lhs0)
        .filter(|(_, wannabe, _)| *wannabe != rhs0)
        .is_none()
        && corrections
            .iter()
            .find(|(_, _, real)| *real == lhs1)
            .filter(|(_, wannabe, _)| *wannabe != rhs1)
            .is_none()
    {
        let mut new_corrections = corrections.to_vec();
        new_corrections.push((sym0, lhs0, rhs0));
        new_corrections.push((sym1, lhs1, rhs1));
        let new_priority = base_priority as Priority + priority_increment;
        corrections_queue.push((new_priority, new_corrections));
    }
}

fn encoded(wire: &str) -> Wire {
    assert!(wire.len() == 3);
    assert!(wire.is_ascii());
    let bytes = wire.bytes().collect::<Vec<u8>>();
    (bytes[0], bytes[1], bytes[2])
}

fn decoded(wire: Wire) -> String {
    let left = wire.0 as char;
    let middle = wire.1 as char;
    let right = wire.2 as char;
    format!("{}{}{}", left, middle, right)
}

fn starts_with(wire: Wire, ch: u8) -> bool {
    let left = wire.0;
    left == ch
}

fn encoded_sorted_inputs(input_1: &str, input_2: &str) -> [Wire; 2] {
    let mut inputs = [encoded(input_1), encoded(input_2)];
    inputs.sort();
    inputs
}

fn sorted_inputs(input_1: Wire, input_2: Wire) -> [Wire; 2] {
    let mut inputs = [input_1, input_2];
    inputs.sort();
    inputs
}

fn depth_from_symbolic_encoded_wire(symbolic_wire: SymbolicWire) -> Depth {
    let middle = symbolic_wire.1;
    let right = symbolic_wire.2;
    ((middle - b'0') * 10) + (right - b'0')
}

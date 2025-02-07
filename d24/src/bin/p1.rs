use std::{
    collections::{BinaryHeap, HashMap},
    ops::Not,
};

#[derive(Clone, Copy)]
enum Gate {
    And,
    Or,
    Xor,
}

type Wire = (u8, u8, u8);
type WireStates = HashMap<Wire, bool>;
type Circuit = HashMap<Wire, (Gate, Wire, Wire)>;

fn main() {
    let mut wire_states: WireStates = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .take_while(|line| line.is_empty().not())
        .map(|line| {
            let (lhs, rhs) = line
                .split_once(": ")
                .expect("No ': ' pattern in first section of input");
            (lhs.to_owned(), rhs.to_owned())
        })
        .map(|(wire, state)| {
            (
                encode(&wire),
                match state.as_str() {
                    "0" => false,
                    "1" => true,
                    _ => panic!("Invalid input state, only 1 and 0 are valid"),
                },
            )
        })
        .collect();
    let circuit: Circuit = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            let (lhs, rhs) = line
                .split_once(" -> ")
                .expect("no ' -> ' pattern in second section of input");
            let output = encode(rhs);
            let lhs = lhs.split_whitespace().collect::<Vec<_>>();
            let gate = match lhs[1] {
                "AND" => Gate::And,
                "OR" => Gate::Or,
                "XOR" => Gate::Xor,
                _ => panic!("Invalid gate type"),
            };
            let input_1 = encode(lhs[0]);
            let input_2 = encode(lhs[2]);
            (output, (gate, input_1, input_2))
        })
        .collect();
    let wires: BinaryHeap<Wire> = circuit
        .keys()
        .filter(|wire| starts_with_z(**wire))
        .cloned()
        .collect();
    let output = wires
        .into_sorted_vec()
        .into_iter()
        .map(|z_wire| wire_state(&circuit, &mut wire_states, z_wire))
        .map(|z_wire| match z_wire {
            true => 1,
            false => 0,
        })
        .enumerate()
        .map(|(idx, z_wire)| z_wire << idx)
        .sum::<u64>();
    println!("{}", output);
}

fn wire_state(circuit: &Circuit, wire_states: &mut WireStates, wire: Wire) -> bool {
    if let Some(&wire_state) = wire_states.get(&wire) {
        return wire_state;
    }
    let (gate, input_1, input_2) = circuit.get(&wire).expect("Did not find wire in circuit");
    let input_1 = wire_state(circuit, wire_states, *input_1);
    let input_2 = wire_state(circuit, wire_states, *input_2);
    let wire_state = match gate {
        Gate::And => input_1 && input_2,
        Gate::Or => input_1 || input_2,
        Gate::Xor => input_1 ^ input_2,
    };
    wire_states.insert(wire.to_owned(), wire_state);
    wire_state
}

fn encode(wire: &str) -> Wire {
    assert!(wire.len() == 3);
    assert!(wire.is_ascii());
    let bytes = wire.bytes().collect::<Vec<u8>>();
    (bytes[0], bytes[1], bytes[2])
}

fn starts_with_z(wire: Wire) -> bool {
    wire.0 == b'z'
}

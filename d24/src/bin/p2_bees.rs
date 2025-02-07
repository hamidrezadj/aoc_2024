use rand::prelude::*;
use rand::rngs::StdRng;
use std::{
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
    ops::Not,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Gate {
    And,
    Or,
    Xor,
}

type Wire = (u8, u8, u8);
type Inputs = [Wire; 2];
// These are BTreeMap instead of HashMap for reproducibility.
// HashMaps are generally faster but accessing them is not deterministic.
type Circuit = BTreeMap<Wire, (Gate, Inputs)>;
type RCircuit = BTreeMap<(Gate, Inputs), Wire>;
type Bee = Vec<Wire>;
type Score = u32;

struct Info {
    r_circuit: RCircuit,
    bits: Score,
    scs: HashSet<Inputs>,
    abs: HashSet<Inputs>,
}

const ELITES: usize = 1;
const ELITES_FOLLOWING: usize = 50;
const DRONES: usize = 10;
const DRONES_FOLLOWING: usize = 5;
const SCOUTS: usize = 50;

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

    let r_circuit: RCircuit = circuit
        .clone()
        .into_iter()
        .map(|(output, (gate, inputs))| ((gate, inputs), output))
        .collect();

    let biggest_z = circuit
        .keys()
        .filter(|wire| starts_with(**wire, b'z'))
        .max()
        .copied()
        .expect("No output that starts with z");
    let bits = decoded(biggest_z)[1..3]
        .parse::<Score>()
        .expect("z output with invalid numeric suffix")
        + 1;

    let all_wires: Vec<Wire> = circuit.keys().cloned().collect();
    assert!(all_wires.is_empty().not(), "Empty input");

    let scs: HashSet<Inputs> = all_wires
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
    let abs: HashSet<Inputs> = all_wires
        .iter()
        .copied()
        .map(|wire| circuit.get(&wire).unwrap())
        .filter(|(gate, _inputs)| gate == &Gate::Or)
        .map(|(_gate, inputs)| inputs.to_owned())
        .collect();

    let info = Info {
        r_circuit,
        bits,
        scs,
        abs,
    };

    let mut rng: StdRng = std::env::args()
        .nth(1)
        .map(|seed| seed.parse::<u64>().expect("Bad seed integer"))
        .map(SeedableRng::seed_from_u64)
        .unwrap_or_else(|| {
            let random_seed = random();
            println!("random seed: {}", random_seed);
            SeedableRng::seed_from_u64(random_seed)
        });

    let mut bees: BinaryHeap<(Score, Bee)> = (0..ELITES + DRONES + SCOUTS)
        .map(|_| {
            let bee = all_wires
                .choose_multiple(&mut rng, 8)
                .cloned()
                .collect::<Bee>();
            let score = bee_score(&info, &bee);
            (score, bee)
        })
        .collect();
    let bee = loop {
        let mut bee_iterator = bees.into_sorted_vec().into_iter().rev();
        let elites: BinaryHeap<(Score, Bee)> = bee_iterator
            .by_ref()
            .take(ELITES)
            .map(|(elite_score, elite)| {
                (0..ELITES_FOLLOWING)
                    .map(|_drone_idx| {
                        let drone = slightly_randomize_bee(&all_wires, &elite, &mut rng);
                        let score = bee_score(&info, &drone);
                        (score, drone)
                    })
                    .max()
                    .unwrap()
                    .max((elite_score, elite))
            })
            .collect();
        let mut drones: BinaryHeap<(Score, Bee)> = bee_iterator
            .by_ref()
            .take(DRONES)
            .map(|(elite_score, elite)| {
                (0..DRONES_FOLLOWING)
                    .map(|_drone_idx| {
                        let drone = slightly_randomize_bee(&all_wires, &elite, &mut rng);
                        let score = bee_score(&info, &drone);
                        (score, drone)
                    })
                    .max()
                    .unwrap()
                    .max((elite_score, elite))
            })
            .collect();
        let mut scouts: BinaryHeap<(Score, Bee)> = (0..SCOUTS)
            .map(|_| {
                let scout = all_wires
                    .choose_multiple(&mut rng, 8)
                    .cloned()
                    .collect::<Bee>();
                let score = bee_score(&info, &scout);
                (score, scout)
            })
            .collect();
        bees = elites;
        bees.append(&mut drones);
        bees.append(&mut scouts);
        let (best_score, best_bee) = bees.peek().unwrap();
        let target_score = info.bits * 3;
        // println!("{} / {}", best_score, target_score);
        if *best_score == target_score {
            break best_bee.clone();
        }
    };
    let output = bee
        .into_iter()
        .map(decoded)
        .collect::<BinaryHeap<_>>()
        .into_sorted_vec()
        .join(",");
    println!("{}", output);
}

fn slightly_randomize_bee(all_wires: &[Wire], leader: &Bee, rng: &mut impl Rng) -> Bee {
    let mut new_bee = leader.clone();
    let remove_idx = rng.gen_range(0..new_bee.len() / 2);
    new_bee[remove_idx * 2] = Default::default();
    new_bee[remove_idx * 2 + 1] = Default::default();
    loop {
        let new_wire_0 = all_wires.choose(rng).copied().unwrap();
        let new_wire_1 = all_wires.choose(rng).copied().unwrap();
        if new_wire_0 == new_wire_1
            || new_bee.contains(&new_wire_0)
            || new_bee.contains(&new_wire_1)
        {
            continue;
        }
        new_bee[remove_idx * 2] = new_wire_0;
        new_bee[remove_idx * 2 + 1] = new_wire_1;
        break;
    }
    new_bee
}

fn bee_score(info: &Info, bee: &[Wire]) -> Score {
    let corrections: HashMap<Wire, Wire> =
        bee.windows(2)
            .step_by(2)
            .fold(HashMap::new(), |mut acc, wires| {
                acc.insert(wires[0], wires[1]);
                acc.insert(wires[1], wires[0]);
                acc
            });
    let z00 = info
        .r_circuit
        .get(&(Gate::Xor, encoded_sorted_inputs("x00", "y00")))
        .copied()
        .unwrap();
    let z00 = corrected(&corrections, z00);
    if z00 != encoded("z00") {
        return 2;
    }
    let mut last_c: Wire = Default::default();
    for depth in 1..info.bits - 1 {
        let xn = encoded(&format!("x{:02}", depth));
        let yn = encoded(&format!("y{:02}", depth));

        let sn = info
            .r_circuit
            .get(&(Gate::Xor, sorted_inputs(xn, yn)))
            .copied()
            .unwrap();
        let sn = corrected(&corrections, sn);
        let cn_1 = if depth == 1 {
            info.r_circuit
                .get(&(Gate::And, encoded_sorted_inputs("x00", "y00")))
                .copied()
                .unwrap()
        } else {
            last_c
        };
        let cn_1 = corrected(&corrections, cn_1);
        if info.scs.contains(&sorted_inputs(sn, cn_1)).not() {
            return depth * 3;
        }

        let an = info
            .r_circuit
            .get(&(Gate::And, sorted_inputs(xn, yn)))
            .copied()
            .unwrap();
        let an = corrected(&corrections, an);
        let bn = info
            .r_circuit
            .get(&(Gate::And, sorted_inputs(sn, cn_1)))
            .copied()
            .unwrap();
        let bn = corrected(&corrections, bn);
        if info.abs.contains(&sorted_inputs(an, bn)).not() {
            return depth * 3 + 1;
        }

        let zn = info
            .r_circuit
            .get(&(Gate::Xor, sorted_inputs(sn, cn_1)))
            .copied()
            .unwrap();
        let zn = corrected(&corrections, zn);
        if zn != encoded(&format!("z{:02}", depth)) {
            return depth * 3 + 2;
        }

        let cn = info
            .r_circuit
            .get(&(Gate::Or, sorted_inputs(an, bn)))
            .copied()
            .unwrap();
        last_c = cn;
    }
    let zm = last_c;
    let zm = corrected(&corrections, zm);
    if zm != encoded(&format!("z{:02}", info.bits - 1)) {
        return (info.bits - 1) * 3 + 2;
    }
    info.bits * 3
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

fn corrected(corrections: &HashMap<Wire, Wire>, wire: Wire) -> Wire {
    match corrections.get(&wire) {
        Some(&wire) => wire,
        None => wire,
    }
}

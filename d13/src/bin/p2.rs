fn main() {
    type InputSegment = Vec<(usize, String)>;
    let (a_coefficients, rest): (InputSegment, InputSegment) = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .enumerate()
        .partition(|(idx, _line)| idx % 4 == 0);
    let (b_coefficients, rest): (InputSegment, InputSegment) =
        rest.into_iter().partition(|(idx, _line)| idx % 4 == 1);
    let (goals, _rest): (InputSegment, InputSegment) =
        rest.into_iter().partition(|(idx, _line)| idx % 4 == 2);
    let a_coefficients = a_coefficients.into_iter().map(|(_idx, line)| {
        let (_lhs, rhs) = line
            .split_once("Button A: X+")
            .expect("Unexpected pattern: Wrong beginning pattern");
        let (x, y) = rhs
            .split_once(", Y+")
            .expect("Unexpected pattern: Wrong middle pattern");
        let x = x
            .parse::<i64>()
            .expect("Not an integer that fits in 64 bits");
        let y = y
            .parse::<i64>()
            .expect("Not an integer that fits in 64 bits");
        (x, y)
    });
    let b_coefficients = b_coefficients.into_iter().map(|(_idx, line)| {
        let (_lhs, rhs) = line
            .split_once("Button B: X+")
            .expect("Unexpected pattern: Wrong beginning pattern");
        let (x, y) = rhs
            .split_once(", Y+")
            .expect("Unexpected pattern: Wrong middle pattern");
        let x = x
            .parse::<i64>()
            .expect("Not an integer that fits in 64 bits");
        let y = y
            .parse::<i64>()
            .expect("Not an integer that fits in 64 bits");
        (x, y)
    });
    let goals = goals.into_iter().map(|(_idx, line)| {
        let (_lhs, rhs) = line
            .split_once("Prize: X=")
            .expect("Unexpected pattern: Wrong beginning pattern");
        let (x, y) = rhs
            .split_once(", Y=")
            .expect("Unexpected pattern: Wrong middle pattern");
        let x = x
            .parse::<i64>()
            .expect("Not an integer that fits in 64 bits");
        let x = x.checked_add(10000000000000).expect("Overflow");
        let y = y
            .parse::<i64>()
            .expect("Not an integer that fits in 64 bits");
        let y = y.checked_add(10000000000000).expect("Overflow");
        (x, y)
    });
    fn determinent((d00, d01): (i64, i64), (d10, d11): (i64, i64)) -> i64 {
        let l = d00.checked_mul(d11).expect("Overflow");
        let r = d01.checked_mul(d10).expect("Overflow");
        l.checked_sub(r).expect("Overflow")
    }
    type System = (((i64, i64), (i64, i64)), (i64, i64));
    fn solve(((a, b), g): System) -> Option<(i64, i64)> {
        let den = determinent(a, b);
        if den == 0 {
            return None;
        }
        let a_nom = determinent(g, b);
        let a_sol = a_nom / den;
        let b_nom = determinent(a, g);
        let b_sol = b_nom / den;
        // A and B solutions need to be non negative integers.
        if a_nom % den != 0 || a_sol < 0 || b_nom % den != 0 || b_sol < 0 {
            return None;
        }
        Some((a_sol, b_sol))
    }
    let output = a_coefficients
        .zip(b_coefficients)
        .zip(goals)
        .filter_map(solve)
        .map(|(a, b)| {
            a.checked_mul(3)
                .and_then(|t| t.checked_add(b))
                .expect("Overflow")
        })
        .reduce(|acc, e| acc.checked_add(e).expect("Overflow"))
        .expect("Empty input");
    println!("{}", output);
}

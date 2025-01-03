use std::{env, io};

fn main() {
    let x_len = env::var("X_LEN")
        .map(|x| {
            x.parse::<i64>()
                .expect("X_LEN environment variable not a 64 bit integer")
        })
        .unwrap_or(101);
    let y_len = env::var("Y_LEN")
        .map(|x| {
            x.parse::<i64>()
                .expect("Y_LEN environment variable not a 64 bit integer")
        })
        .unwrap_or(103);
    if x_len < 1 || y_len < 1 {
        panic!("Length or width of the room can't be non-positive");
    }
    let output = io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            let (p_segment, v_segment) = line.split_once(' ').expect("No space in line");
            let (_rest, p_numbers) = p_segment
                .split_once("p=")
                .expect("No p= pattern in position segment");
            let (px, py) = p_numbers
                .split_once(',')
                .expect("No comma in position numbers segment");
            let (_rest, v_numbers) = v_segment
                .split_once("v=")
                .expect("No v= pattern in velocity segment");
            let (vx, vy) = v_numbers
                .split_once(',')
                .expect("No comma in velocity numbers segment");
            let px = px
                .parse::<i64>()
                .expect("Not an integer that fits in 64 bits");
            let py = py
                .parse::<i64>()
                .expect("Not an integer that fits in 64 bits");
            let vx = vx
                .parse::<i64>()
                .expect("Not an integer that fits in 64 bits");
            let vy = vy
                .parse::<i64>()
                .expect("Not an integer that fits in 64 bits");
            ((px, py), (vx, vy))
        })
        .map(|((position_x, position_y), (velocity_x, velocity_y))| {
            let delta_x = velocity_x * 100;
            let delta_y = velocity_y * 100;
            let x = position_x + delta_x;
            let y = position_y + delta_y;
            let x = x.rem_euclid(x_len);
            let y = y.rem_euclid(y_len);
            (x, y)
        })
        .fold([0usize; 4], |mut quadrants_count, (x, y)| {
            let x_half = x_len / 2;
            let y_half = y_len / 2;
            if x < x_half && y < y_half {
                quadrants_count[0] += 1;
            } else if x > x_half && y < y_half {
                quadrants_count[1] += 1;
            } else if x < x_half && y > y_half {
                quadrants_count[2] += 1;
            } else if x > x_half && y > y_half {
                quadrants_count[3] += 1;
            }
            quadrants_count
        })
        .into_iter()
        .reduce(|acc, q| acc * q)
        .expect("Empty input");
    println!("{}", output);
}

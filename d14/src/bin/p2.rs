use std::{collections::BinaryHeap, env, io};

fn main() {
    let mut args = env::args().skip(1).take(2);
    let control_argument = args.next();
    let target_time = args.next();
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
    let input: Vec<((i64, i64), (i64, i64))> = io::stdin()
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
        .collect();
    match control_argument {
        Some(ca) if ca == "draw-until" => {
            let target_time = target_time
                .expect("No time target argument passed")
                .parse::<i64>()
                .expect("Time target argument not an integer that fits in 64 bits");
            let mut input = input;
            for time in 1..=target_time {
                let mut buffer: BinaryHeap<(i64, i64)> = BinaryHeap::new();
                input.iter_mut().for_each(|((px, py), (vx, vy))| {
                    *px = (*px + *vx).rem_euclid(x_len);
                    *py = (*py + *vy).rem_euclid(y_len);
                    buffer.push((*py, *px));
                });
                draw(x_len, time, buffer);
            }
        }
        Some(ca) if ca == "draw" => {
            let time = target_time
                .expect("No time target argument passed")
                .parse::<i64>()
                .expect("Time target argument not an integer that fits in 64 bits");
            let buffer: BinaryHeap<(i64, i64)> = input
                .iter()
                .map(|((px, py), (vx, vy))| {
                    let dx = vx * time;
                    let dy = vy * time;
                    let x = (px + dx).rem_euclid(x_len);
                    let y = (py + dy).rem_euclid(y_len);
                    (y, x)
                })
                .collect();
            draw(x_len, time, buffer);
        }
        Some(_) => panic!("Invalid argument: Either use 'draw-until time' or 'draw time'"),
        None => println!("{}", 103 * 75 + 65),
    }
}

fn draw(x_len: i64, time: i64, buffer: BinaryHeap<(i64, i64)>) {
    println!("{}", "---".repeat(x_len as usize));
    println!("t={}", time);
    print!("\n\n\n\n");
    let mut current_y = 0;
    let mut current_x = 0;
    buffer.into_sorted_vec().into_iter().for_each(|(y, x)| {
        if (x, y) == (current_x, current_y) {
            return;
        }
        if y > current_y {
            current_x = 0;
            for _ in current_y..y {
                println!();
            }
            current_y = y;
        }
        for _ in current_x..x - 1 {
            print!("   ");
        }
        print!("###");
        current_x = x;
    });
    print!("\n\n\n\n");
    println!("{}", "---".repeat(x_len as usize));
}

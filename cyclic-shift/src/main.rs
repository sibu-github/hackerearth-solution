use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let test_cases = input.trim().parse::<usize>().unwrap();
    for _ in 0..test_cases {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let line = input
            .trim()
            .split_whitespace()
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let n = line[0];
        let k = line[1];

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut max_val_reached = 1_usize;
        let input = input.trim();
        let mut max_val = input.to_string();
        (1..n).for_each(|n| {
            let s = format!("{}{}", &input[n..], &input[..n]);
            if max_val == s {
                max_val_reached += 1;
            } else if s > max_val {
                max_val = s;
                max_val_reached = 1;
            }
        });
        let (full_cycle, mut remaining_k) = if k % max_val_reached == 0 {
            if k < max_val_reached {
                (0, k)
            } else {
                (k / max_val_reached - 1, max_val_reached)
            }
        } else {
            (k / max_val_reached, k % max_val_reached)
        };
        let mut counter = 0_usize;
        loop {
            let idx = counter % n;
            let s = format!("{}{}", &input[idx..], &input[..idx]);
            if s == max_val {
                remaining_k -= 1;
            }
            if remaining_k == 0 {
                break;
            }
            counter += 1;
        }
        let counter = full_cycle * n + counter;
        println!("{}", counter);
    }
}

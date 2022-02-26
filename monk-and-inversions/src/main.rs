use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("not able to read input");
    let test_cases = input.trim().parse::<usize>().unwrap();
    for _ in 0..test_cases {
        process_input();
    }
}

fn process_input() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("not able to read input");
    let size = input.trim().parse::<usize>().unwrap();
    let rows = (0..size)
        .map(|_| {
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("not able to read input");
            input
                .split_whitespace()
                .map(|s| s.trim().parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    let count = rows.iter().enumerate().fold(0, |acc, (idx, &elem)| {
        let c = rows
            .iter()
            .enumerate()
            .filter(|&(i, &v)| i > idx && i % size >= idx % size && v < elem)
            .count();

        acc + c
    });
    println!("{}", count);
}

use std::collections::HashMap;
use std::io;

const UPPER_LIMIT: u64 = 1_000_000_009;

fn main() {
    let mut map = HashMap::<u64, u64>::new();
    map.insert(0, 1);
    map.insert(1, 10);
    map.insert(2, 99);
    map.insert(3, 980);
    map.insert(4, 9701);

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let test_cases = input.trim().parse::<usize>().unwrap();
    (0..test_cases).for_each(|_| {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n = input.trim().parse::<u64>().unwrap();
        println!("{}", solve(n, &mut map));
    });
}

fn solve(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&v) = map.get(&n) {
        return v;
    }

    let p1 = solve(n / 2, map);
    let p2 = solve((n / 2) - 1, map);
    if n % 2 == 0 {
        let v1 = (p1 * p1) % UPPER_LIMIT;
        let v2 = (p2 * p2) % UPPER_LIMIT;
        let v = handle_subtract_overflow(v1, v2);
        map.insert(n, v);
        return v;
    }
    let p3 = solve((n / 2) + 1, map);
    let p4 = handle_subtract_overflow(p3, p2);
    let v = (p1 * p4) % UPPER_LIMIT;
    map.insert(n, v);
    v
}

fn handle_subtract_overflow(v1: u64, v2: u64) -> u64 {
    let v = (v1 as i64 - v2 as i64) % UPPER_LIMIT as i64;
    if v < 0 {
        (v + UPPER_LIMIT as i64) as u64
    } else {
        v as u64
    }
}

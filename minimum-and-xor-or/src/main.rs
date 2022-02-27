use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let test_cases = input.trim().parse::<usize>().unwrap();
    for _ in 0..test_cases {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n = input.trim().parse::<usize>().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut arr = input
            .trim()
            .split_whitespace()
            .take(n)
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        arr.sort();
        let v = arr
            .windows(2)
            .map(|e| (e[0] & e[1]) ^ (e[0] | e[1]))
            .min()
            .unwrap();
        println!("{}", v);
    }
}

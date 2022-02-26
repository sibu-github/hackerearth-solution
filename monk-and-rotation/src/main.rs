use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let test_cases = input.trim().parse::<u32>().unwrap();
    for _ in 0..test_cases {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let line = input
            .trim()
            .split_whitespace()
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let n = line[0];
        let k = line[1] % n;

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let arr = input.trim().split_whitespace().collect::<Vec<_>>();
        let arr = arr
            .iter()
            .chain(arr.iter())
            .skip(n - k)
            .take(n)
            .cloned()
            .collect::<Vec<_>>();

        println!("{}", arr.join(" "));
    }
}

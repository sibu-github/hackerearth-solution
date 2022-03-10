use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input
        .trim()
        .split_whitespace()
        .map(|s| s.trim())
        .collect::<Vec<_>>();
    let s = input[0];
    let k = input[1].parse::<usize>().unwrap();
    let mut suffixes = vec![];
    for i in 0..s.len() {
        suffixes.push(s[i..].to_string());
    }
    suffixes.sort();
    println!("{}", suffixes[k - 1]);
}

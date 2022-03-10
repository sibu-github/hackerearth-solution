fn main() {
    // let mut size = 3_usize;
    let mut a = "11 12 301"
        .split_whitespace()
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut b = "12 301 11"
        .split_whitespace()
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut total_move = 0;

    loop {
        if a.iter().eq(&b) {
            total_move += a.len();
            break;
        }
        if a[0] == b[0] {
            total_move += 1;
            a = a[1..].to_vec();
            b = b[1..].to_vec();
            continue;
        }
        let t = a[0];
        a = a[1..].to_vec();
        a.push(t);
        total_move += 1;
    }

    println!("{}", total_move);
}

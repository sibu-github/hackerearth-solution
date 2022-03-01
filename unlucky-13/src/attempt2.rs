use std::collections::HashMap;
use std::io;

const UPPER_LIMIT: u64 = 1_000_000_009;

fn main() {
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    // let test_cases = input.trim().parse::<usize>().unwrap();
    let n = 2_865_406;
    let mut terms13_map = HashMap::<u64, u64>::new();
    // (0..test_cases).for_each(|_| {
    //     input.clear();
    //     io::stdin().read_line(&mut input).unwrap();
    //     let n = input.trim().parse::<u64>().unwrap();
    //     if n == 0 {
    //         println!("0");
    //         return;
    //     }
    build_cache(n, &mut terms13_map);
    //     let total_possiblity = calc_pow(10, n);
    //     let with_13 = get_with_13(n, &mut terms13_map);
    //     println!(
    //         "{} {} {}",
    //         handle_subtract_overflow(total_possiblity, with_13),
    //         total_possiblity,
    //         with_13
    //     );
    // });
    // println!("{}", with_13);
}

fn build_cache(n: u64, map: &mut HashMap<u64, u64>) {
    (1..=n).for_each(|n| {
        get_with_13(n, map);
    });
}

fn handle_subtract_overflow(t1: u64, t2: u64) -> u64 {
    let v = (t1 as i64 - t2 as i64) % UPPER_LIMIT as i64;
    if v < 0 {
        (v + UPPER_LIMIT as i64) as u64
    } else {
        v as u64
    }
}

fn get_with_13(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    match n {
        0 => 0,
        1 => 0,
        2 => 1,
        _ => {
            if let Some(&v) = map.get(&n) {
                v
            } else {
                let t1 = (20 * get_with_13(n - 1, map)) % UPPER_LIMIT;
                let t2 = (101 * get_with_13(n - 2, map)) % UPPER_LIMIT;
                let t3 = (10 * get_with_13(n - 3, map)) % UPPER_LIMIT;
                let v = handle_subtract_overflow(t1 + t3, t2);
                map.insert(n, v);
                v
            }
        }
    }
}

fn calc_pow(m: u64, e: u64) -> u64 {
    if e == 0 {
        return 1;
    }
    if e == 1 {
        return m;
    }
    calc_mod_pow(m, e, UPPER_LIMIT)
}

// calculate m^e % n
fn calc_mod_pow(m: u64, e: u64, n: u64) -> u64 {
    // get binary representation of the exponent
    let bstring = format!("{:b}", e);
    let mut d = 1_u64;
    // perform d^2 if 0 found, d^2 * d*m if 1 found
    bstring.chars().for_each(|ch| match ch {
        '1' => {
            d = d.pow(2) % n;
            d = (d * m) % n;
        }
        _ => d = d.pow(2) % n,
    });
    d
}

// 20𝑐𝑛−1−101𝑐𝑛−2+10𝑐𝑛−3

// C0 -- 0
// C1 -- 0
// C2 -- 1
// C3 = 20C2 - 101C1 + 10C0 = 20 - 0 + 0 = 20
// C4 = 20C3 - 101C2 + 10C1 = 20*20 - 101 + 0 = 299
// C5 = 20C4 - 101C3 + 10C2 = 20*299 - 101*20 + 10 = 3970
// C6 = 20C5 - 101C4 + 10C3 = 20*3970 - 101*299 + 10*20 = 49401

// fn get_with_13(n: u64) -> u64 {
//     if n == 0 || n == 1 {
//         return 0;
//     }

//     // https://math.stackexchange.com/a/924835/601822
//     // Cn = 10^n − ((5 + √24)^(n + 1))/2√24)+ ((5 - √24)^(n + 1))/2√24)
//     // let t1 = calc_pow_f64(10_f64, n);

//     // (2*√24)(((5 + √24)^(n+1)) - ((5 - √24)^(n+1)))
//     let m1 = 2_f64 * 24_f64.sqrt();
//     let r1 = 5_f64 + 24_f64.sqrt();
//     let r2 = 5_f64 - 24_f64.sqrt();
//     let t1 = calc_pow_f64(r1, n + 1);
//     let t2 = calc_pow_f64(r2, n + 1);
//     let t = ((t1 - t2) / m1) % UPPER_LIMIT as f64;
//     let t = t.round() as u64;
//     t
// }

// fn calc_pow_f64(m: f64, e: u64) -> f64 {
//     let upper_limit = UPPER_LIMIT as f64;
//     let bstring = format!("{:b}", e);
//     let d = bstring.chars().fold(1_f64, |acc, ch| match ch {
//         '1' => {
//             let d = (acc * acc) % upper_limit;
//             (d * m) % upper_limit
//         }
//         _ => (acc * acc) % upper_limit,
//     });

//     d
// }

use std::io;

const UPPER_LIMIT: u64 = 1_000_000_009;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let test_cases = input.trim().parse::<usize>().unwrap();
    (0..test_cases).for_each(|_| {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n = input.trim().parse::<u64>().unwrap();
        if n == 0 {
            println!("0");
            return;
        }

        let total_possiblity = calc_pow(10, n);
        let with_13 = get_with_13_terms(n);
        // let with_13t = get_with_13_terms(n);
        println!(
            "{} {} {}",
            total_possiblity - with_13,
            total_possiblity,
            with_13
        );
    });
    // let t = get_with_13_terms(6);
    // println!("{}", t);
}

fn get_with_13(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 0;
    }

    // https://math.stackexchange.com/a/924835/601822
    // Cn = 10^n − ((5 + √24)^(n + 1))/2√24)+ ((5 - √24)^(n + 1))/2√24)
    let t1 = calc_pow_f64(10_f64, n);
    let r1 = 5_f64 + 24_f64.sqrt();
    let r2 = 5_f64 - 24_f64.sqrt();
    let t2 = calc_pow_f64(r1, n + 1);
    let t2 = t2 / (2_f64 * 24_f64.sqrt());
    let t3 = calc_pow_f64(r2, n + 1);
    let t3 = t3 / (2_f64 * 24_f64.sqrt());
    let t = (t1 - t2 + t3) % UPPER_LIMIT as f64;
    t as u64
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

fn calc_pow_f64(m: f64, e: u64) -> f64 {
    let upper_limit = UPPER_LIMIT as f64;
    let bstring = format!("{:b}", e);
    let mut d = 1_f64;
    bstring.chars().for_each(|ch| match ch {
        '1' => {
            d = d.powf(2_f64) % upper_limit;
            d = (d * m) % upper_limit;
        }
        _ => {
            d = d.powf(2_f64);
        }
    });

    d
}

fn ncr(n: u64, r: u64) -> u64 {
    (factorial(n) / (factorial(r) * factorial(n - r))) as u64 % UPPER_LIMIT
}

fn factorial(n: u64) -> u64 {
    // let mut r = 1;
    // (0..n).for_each(|x| {
    //     r = (r * (x + 1)) % UPPER_LIMIT;
    // });
    // r
    let p = (1..=n).product::<u64>();
    p % UPPER_LIMIT
}

fn get_with_13_terms(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 0;
    }

    let mut result = 0;
    for k in 1..=(n / 2) {
        let t = ncr(n - k, k);
        let p = calc_pow(10, n - (2 * k));
        if k % 2 == 0 {
            result = (result - (p * t)) % UPPER_LIMIT;
        } else {
            result = (result + (p * t)) % UPPER_LIMIT;
        }
    }

    result
}

// 20𝑐𝑛−1−101𝑐𝑛−2+10𝑐𝑛−3

// C0 -- 0
// C1 -- 0
// C2 -- 1
// C3 = 20C2 - 101C1 + 10C0 = 20 - 0 + 0 = 20
// C4 = 20C3 - 101C2 + 10C1 = 20*20 - 101 + 0 = 299
// C5 = 20C4 - 101C3 + 10C2 = 20*299 - 101*20 + 10 = 3970
// C6 = 20C5 - 101C4 + 10C3 = 20*3970 - 101*299 + 10*20 = 49401

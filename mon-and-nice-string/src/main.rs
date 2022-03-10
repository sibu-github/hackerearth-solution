use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    
    let mut arr = (0..n).map(|i|{
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        (i, input.trim().to_string())
    }).collect::<Vec<(usize, String)>>();
    arr.sort_by(|a, b| {
        a.1.cmp(&b.1)
    });
    println!("{:?}", arr);
    let mut v = vec![0_usize; arr.len()];
    arr.iter().enumerate().for_each(|(i, e)|{
        let c = arr.iter().take(i).filter(|&p| {
            p.0 < e.0 && p.1 != e.1 
        }).count();
        v[e.0] = c;
    });
    for i in v {
        println!("{}", i);
    }
}

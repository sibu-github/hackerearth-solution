

   // let mut start = 1;
    // let mut all_chosen = vec![];
    // loop {
    //     if start > size1 {
    //         break;
    //     }

    //     let mut chosen_a = vec![];
    //     let mut chosen_b = vec![];
    //     let mut prev_a = 0; 
    //     let mut prev_b = 0;
    //     let mut skip = 0;
    //     for i in start..=size1 {
    //         if let Some(idx) = b.iter().skip(skip).position(|&x| x == a[i]) {
    //             let idx = idx + skip;
    //             println!("i={}, idx={}, a[i]={}, b[idx]={}", i, idx, a[i], b[idx]);
    //             if i > prev_a && idx > prev_b &&  a[i] >= a[prev_a] && b[idx] >= b[prev_b] {
    //                 chosen_a.push(i);
    //                 chosen_b.push(idx);
    //                 prev_a = i;
    //                 prev_b = idx;
    //                 skip = if skip == 0 {idx + 1 } else {idx};
    //             }

    //         }
    //     }
    //     all_chosen.push((chosen_a, chosen_b));
    //     start += 1;
    // }
    // println!("{:?}", all_chosen);
    // all_chosen.sort_by(|e1, e2| {
    //     e2.0.len().cmp(&e1.0.len())
    // });

    // let (v1, v2) = &all_chosen[0];
    // let s1 = v1.iter().fold("".to_string(), |acc, &x| format!("{} {}", acc, x));
    // let s2 = v2.iter().fold("".to_string(), |acc, &x| format!("{} {}", acc, x));
    // println!("{}", s1.trim());
    // println!("{}", s2.trim());


fn main() {

    let size1 = 4;
    let size2 = 4; 

    let  a = vec![1, 2, 1, 4];
    let  b = vec![1, 4, 1, 1, 4];
    let mut copy_a = a.clone();
    let mut copy_b = b.clone();

    copy_a.sort_unstable();
    copy_b.sort_unstable();
    copy_a.reverse();


    let mut sub_seq = vec![];
    let mut last = 0;
    loop {
        if let Some(v) = copy_a.pop() {
            if sub_seq.len() > 0 {
                if copy_b[last + 1] == v {
                    last += 1;
                } else {
                    last = 0;
                    sub_seq = vec![];
                }
            } else {
                if let Some(idx) = copy_b.iter().position(|&x| x == v) {
                    
                }
            }

            



  
        } else {
            break;
        }

    }








    println!("{:?}", a);
    println!("{:?}", b);




 
}

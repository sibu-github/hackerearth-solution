mod node;
use std::io;





fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim().to_string();

    let mut t = node::Tree::new();
    s.char_indices().for_each(|(idx, ch)|{
        t.push(ch, idx + 1);
    });

    println!("{:#?}", t);

}


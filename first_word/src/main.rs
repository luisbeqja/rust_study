fn main() {
    let mut s = String::from("testss");
    let word = first_word(&s);
    s.clear();
    println!("{}", word)
}

fn first_word(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

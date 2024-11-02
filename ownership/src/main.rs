#[allow(unused_variables)]
fn main() {
    let mut s1 = String::from("Hello world");
    let word_ = first_word(&mut s1);
    // takes_ownership(&s1);
    // println!("{s1}");
    println!("{}", word_);
    s1.clear();

    // let hello = &s1[0..5];
    // println!("{}", hello.len());
}

#[allow(dead_code)]
fn takes_ownership(s1: &mut String) {
    println!("{s1}");
}

#[allow(dead_code, unused_variables)]
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

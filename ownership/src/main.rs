fn main() {
    let s1 = String::from("Hello world");
    takes_ownership(&s1);
    println!("{s1}");
}

fn takes_ownership(s1: &String) {
    println!("{s1}");
}

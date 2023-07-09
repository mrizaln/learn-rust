use std::ops::Deref;

pub fn main() {
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    s2.push('!');
    println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());

    let s3 = String::from("🇨🇭");
    println!(
        "s2: len = {}, number of chars = {}",
        s3.len(),
        s3.chars().count()
    );

    // idk
    let _s4 = s1.deref();
    let _s5 = &*s1;

    let char_from_s1 = s1.chars().nth(4).unwrap(); // bound checked
    println!("char_from_s1: {char_from_s1}");
    let substr_from_s1 = &s1[0..4]; // bound checked
    println!("substr_from_s1: {substr_from_s1}");
}

pub fn main() {
    let numbers = vec![10, 20, 30];
    let first: Option<&i8> = numbers.first();
    println!("first: {first:?}");
    
    let idx: Result<usize, usize> = numbers.binary_search(&20);
    println!("idx: {idx:?}");
    let idx: Result<usize, usize> = numbers.binary_search(&25);
    println!("idx: {idx:?}");
}

fn apply_with_log(func: impl FnOnce(i32) -> i32, input: i32) -> i32 {
    println!("Calling function on {input}");
    func(input)
}

fn make_greeter(prefix: String) -> impl Fn(&str) {
    return move |name| println!("{} {}", prefix, name);
}

pub fn main() {
    let add_3 = |x| x + 3; // Fn
    println!("add_3(2) = {}", apply_with_log(add_3, 10));
    println!("add_3(2) = {}", apply_with_log(add_3, 20));

    let mut v = Vec::new();
    // FnMut
    let mut accumulate = |x: i32| {
        v.push(x);
        v.iter().sum::<i32>()
    };
    println!("accumulate: {}", apply_with_log(&mut accumulate, 4));
    println!("accumulate: {}", apply_with_log(&mut accumulate, 5));

    let multiply_sum = |x| x * v.into_iter().sum::<i32>(); // FnOnce
    println!("multiply_sum: {}", apply_with_log(multiply_sum, 3));

    let hi = make_greeter("Hi".into());
    hi("there");
}

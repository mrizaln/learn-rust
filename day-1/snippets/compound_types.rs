fn main() {
    // array assignment and access
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {:?}", a); // *)

    // tuple assignment and access
    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0);
    println!("1st index: {}", t.1);
}

// *)    ? means use debug implementation; adding # eg {a:#?} invokes pretty printing

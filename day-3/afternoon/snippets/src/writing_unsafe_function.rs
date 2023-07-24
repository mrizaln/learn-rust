/// Swaps the values pointed to by the given pointers.
///
/// # Safety
///
/// The pointers must be valid and properly aligned.

unsafe fn swap(a: *mut u8, b: *mut u8) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

pub fn main() {
    let mut a = 42;
    let mut b = 66;

    // Safe because ...
    unsafe {
        swap(&mut a, &mut b);
    }

    println!("a = {}, b = {}", a, b);
}

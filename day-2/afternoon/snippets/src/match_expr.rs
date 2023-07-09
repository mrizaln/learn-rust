#[rustfmt::skip]
pub fn main() {
    match std::env::args().next().as_deref() {
        Some("cat") => println!("meow"),
        Some("ls")  => println!("list things"),
        Some("mv")  => println!("move things"),
        Some("rm")  => println!("remove things"),
        None        => println!("no args"),
        _           => println!("unknown command"),
    }
}

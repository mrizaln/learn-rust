mod foo {
    pub fn do_someting() {
        println!("in the foo module");
    }
}

mod bar {
    pub fn do_someting() {
        println!("in the bar module");
    }
}

pub fn main() {
    foo::do_someting();
    bar::do_someting();
}

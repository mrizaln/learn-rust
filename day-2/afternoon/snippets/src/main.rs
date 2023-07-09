pub mod blocks;
pub mod break_and_continue;
pub mod for_loop;
pub mod if_let;
pub mod loop_expr;
pub mod match_expr;
pub mod option_and_result;
pub mod rust_box;
pub mod rust_box_recursive_ds;
pub mod rust_hashmap;
pub mod rust_module;
pub mod rust_module_visibility;
pub mod rust_rc;
pub mod rust_string;
pub mod rust_vec;
pub mod while_let_loop;
pub mod while_loop;

fn main() {
    blocks::main();
    if_let::main();
    while_loop::main();
    while_let_loop::main();
    for_loop::main();
    loop_expr::main();
    match_expr::main();
    break_and_continue::main();
    option_and_result::main();
    rust_string::main();
    rust_vec::main();
    rust_hashmap::main();
    rust_box::main();
    rust_box_recursive_ds::main();
    rust_rc::main();
    rust_module::main();
    rust_module_visibility::main();
}

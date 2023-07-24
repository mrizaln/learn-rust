// Error handling
pub mod add_context_to_error;
pub mod converting_error_types;
pub mod dynamic_error_types;
pub mod error_handling_with_result;
pub mod stack_unwind;
pub mod thiserror_crate;
pub mod try_operator;

// Testing
// none

// Unsafe Rust
pub mod calling_unsafe_function;
pub mod dereferencing_raw_pointer;
pub mod writing_unsafe_function;

fn main() {
    stack_unwind::main();
    error_handling_with_result::main();
    try_operator::main();
    converting_error_types::main();
    thiserror_crate::main();
    dynamic_error_types::main();
    add_context_to_error::main();

    dereferencing_raw_pointer::main();
    calling_unsafe_function::main();
    writing_unsafe_function::main();
}

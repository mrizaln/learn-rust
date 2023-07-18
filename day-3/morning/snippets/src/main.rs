pub mod closures;
pub mod default_methods;
pub mod default_trait;
pub mod deriving_traits;
pub mod drop_trait;
pub mod from_iterator;
pub mod generics;
pub mod impl_trait;
pub mod iterators;
pub mod operator_overload_trait;
pub mod read_and_write;
pub mod trait_bounds;
pub mod trait_objects;
pub mod traits;

fn main() {
    generics::main();
    traits::main();
    trait_objects::main();
    deriving_traits::main();
    default_methods::main();
    trait_bounds::main();
    impl_trait::main();
    iterators::main();
    from_iterator::main();
    let _ = read_and_write::main();
    drop_trait::main();
    default_trait::main();
    operator_overload_trait::main();
    closures::main();
}

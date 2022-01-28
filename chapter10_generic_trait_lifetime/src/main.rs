/**
 * Generic, Trait & Lifetime are for reducing the code duplication.
 */


mod generic;
mod trait_;
mod lifetime;

fn main() {
    generic::main();
    trait_::main();
    lifetime::main();
}

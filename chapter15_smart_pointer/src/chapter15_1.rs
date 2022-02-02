/**
 * Smart pointer is a memory location stores a memory address pointing to some other data in memory. 
 * 1. Most common smart pointer is "&" (reference) --> But Reference does not have other functions unlike smart pointer
 * 2. In most cases, Smart pointer owns the value of the data unlike the "&" which just borrows the data --> String & Vec are both smart pointers
 * 3. Smart pointer are usually implemented by struct. But unlike the normal struct, it implement "deref" & "drop"
 *      - Deref trait allows instances of your smart pointer struct to be treated like references
 *      - Drop trait allows you to customize the code that is run when the instance of your smart pointer goes out of scope
 * 4. Box is a common smart pointer allows you to allocate value on the heap (Following 3 will be covered in depth in chapter 17)
 *      - When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
 *      - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
 *      - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
 * 5. The recursive Cons list is the example of the point#1 above to use Box smart pointer.
 */


/* In following case: ERROR --> recursive type `List` has infinite size
** Reason: Rust compiler does not know how much memory size the "List" take since "List" is a recursive type.

enum List {
    Cons(i32, List),
    Nil
}

ERROR msg:

    error[E0072]: recursive type `List` has infinite size
    --> src/main.rs:27:1
        |
    27  | enum List {
        | ^^^^^^^^^ recursive type has infinite size
    28  |     Cons(i32, List),
        |               ---- recursive without indirection
        |
        help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
        |
    28  |     Cons(i32, Box<List>),
        |               ++++    +

** To fix it, using Box smart pointer to wrap the List type. Then compiler will then know the size as Box is just a "FIX SIZE MEMORY ADDRESS" which point to changable memory heap.
*/

enum List {
    Cons(i32, Box<List>),
    Nil
}
use List::{Cons, Nil};
pub fn main() {
    let list = Cons(5, Box::new(Cons(6, Box::new(Cons(7, Box::new(Nil))))));
}
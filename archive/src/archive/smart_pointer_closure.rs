


/*
** @ Annotation: Smart pointer (Box::new())
** The easest way to explain the box type is using Cons function(Constructor). Cons is to create a list recursively
*/

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    End,
}

use std::fmt::Debug;

use List::Cons;
use List::End;

pub fn cons_demo() {
    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(End))))));
    println!("{:?}", l)
}

pub fn box_demo() {
    let y = 4;
    let x = &y;
    let z = Box::new(y);

    if *x == *z {
        println!("TRUE")
    }
}

/*
** @ Annotation: Closure is an anynomuous function:  |args| { function body }
** Closure is convinent to work with because it borrow the variable in the same scope (Case 1)
** Closure can be passed into another function (Case2) using Fn trait
** Closure can be used in struct (Case3) using generic + Fn trait
** Closure can also be output of a fucntion (Case 4). But rust currently only support output as concrete data type. To make it concrete is using Box type
** Closure is usefal in when we use iterator (Case 5): quit similar as javascript iterator
*/

// Case 1
pub fn closure_demo() {
    let mut c = 0;
    let mut f = || {
        c += 1;
        println!("{}", c)
    };    
    f();
    f();
    f(); 
}

// Case 2

fn runFn<F>(f: F)
    where F: Fn() {
        f()
    }

fn runFnWithArg<F> (f: F) -> i32
    where F: Fn(i32) -> i32 {
        f(3)
    }

pub fn closure2_demo() {
    let demo_fn = || println!("demo function is run");
    runFn(demo_fn);
    let demo_fn2 = |i| i * 10;
    println!("{:?}", runFnWithArg(demo_fn2))
}

// Case 3
#[derive(Debug)]
struct A<F: Fn(i32) -> i32> {
    f: F
}

pub fn closure3_demo() {
    let f = |i: i32 | -> i32 {
        i + 3
    };
    let a = A {f: f};
    // TODO --> How to call the function in struct
}

// Case 4 
fn create() -> Box<Fn()> {
    Box::new(move || println!("This is a closure in Box"))
}

pub fn closure4_demo() {
    let x = create();
    x();
}

// Case 5

pub fn closure5_demo() {
    let v = vec![1,2,3,4,5];
    let has3 = v.iter().any(|&x| x == 3);
    println!("vec has 3: {}", has3)
}

/*
** @ Annotation: Iterator
** Iterator is a built-in trait in rust
*/

trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}


/**
 * @Smart pointers are data structures that act like a pointer but have metadata and actual capability.
 * Smart pointers own the data that they are pointing to --> string and vector
 */

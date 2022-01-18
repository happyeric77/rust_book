

/*
** @ annotation: trait
** Trait is similar as Interface as other languages, they allow as to define what a functions should look like and implement it to other data types
*/

trait Shape {
    fn area(&self) -> u32;
}

struct Rectangle {
    x: u32,
    y: u32,
}

struct Circle {
    r: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.x * self.y
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        (3.1415926 * (self.r * self.r)) as u32
    }
}

pub fn trait_demo() {
    let r = Rectangle {x: 10,y: 10,};
    let c = Circle {r: 10.0};
    println!("{}", r.area());
    println!("{}", c.area())
}


/*
** Compliler is essentially capable of providing basic implementations for specific traits.
** There are many other traits like #[derive(partialEq, Eq, partialOrd, Ord)]
*/

#[derive(Clone, Debug)]
struct A (i32);

#[derive(Clone, Copy, Debug)]
struct A2 (i32);



struct B (f32);

pub fn trait2_demo() {
    let a = A(55);

    // Case 1: this will make the following line print a fail because a is borrowed by c so that we will be no longer move the "a" to println!() function
    // let c = a
    let c = a.clone();
    println!("{:#?}", a);
    // The way to fix is using Clone trait and clone the a to c.
}

pub fn trait3_demo() {
    // Case 2: The will auto copy a to c so that we do not need to clone it everytime.
    // To derieve Copy, we also need to derive Clone at the same time.
    let a = A2(55);
    let c = a;
    println!("{:#?}", a)
}


/*
** @ annotation: We can also use traits to overload operators, so we can also use our binary operator structs.
** There are many other trait to override --> Drop & Iterator ...
*/


use std::ops;

struct A3;

struct B3;


struct A3B3;

struct B3A3;

impl ops::Add<B3> for A3 {
    type Output = A3B3;
    fn add(self, _rhs: B3) -> A3B3 {
        A3B3
    }
}

impl ops::Add<A3> for B3 {
    type Output = B3A3;
    fn add(self, _rhs: A3) -> B3A3 {
        B3A3
    }
}

/*
** Generic type
** Allows us to generalize the type delclartion for specific struct
*/

#[derive(Debug)]
struct Square<T> {
    x: T
}

pub fn generic_demo() {
    let s1 = Square{x: 10};
    let s2 = Square{x: 10.1};
    let s3 = Square{x: 'c'};
    let s4 = Square{x: "string"};
    println!("{:?},{:?},{:?},{:?}", s1, s2, s3, s4)
}


/*
** @ Annotation: <T: fmt::Debug>
** Sort hand of deriving Debug to the generic type T
*/

use std::fmt;

fn generic_fn<T: fmt::Debug>(x: T) {
    println!("{:?}", x)
}

pub fn generic2_demo () {
    generic_fn(10);
    generic_fn(String::from("generic"));
}

/*
** @ Annotation: We can adapt multiple generic types to a struct to set contraint explicitly.
*/

struct A4<U, V> {
    x: U,
    y: V,
}

struct B4<U> {
    x: U,
    y: U,
}


/*
** @ Annotation: We often use generic type with certain trait to make sure the operations and ouput are predictable
*/

use std::ops::Mul;
use std::process::Output;
trait Shape1 <T> {
    fn area(&self) -> T;
}

struct Rectangle1 <T> {
    x: T,
    y: T,
}

impl <T: Mul<Output = T> + Copy> Shape1<T> for Rectangle1<T> {
    fn area(&self) -> T {
        self.x * self.y
    }
}








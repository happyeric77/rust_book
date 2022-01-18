

/*
** @dev datatype
*/
#[allow(dead_code)]
fn tuple_demo() {
    /*
    ** @dev tuple
    */

    // tuple --> integer, char, bool (can contain different data types)
    let t = (1, 'a', false);

    // println!("{}", t); // cannot use this bacuse print cannot rcognize tupple type.

    println!("{:?}", t); // Use {:?} as debug tag;
    println!("{:#?}", t); // Use {:#?} as prettifer;
}
#[allow(dead_code)]
fn array_demo() {
    let xs: [i32; 5] = [1,2,3,4,5];
    println!("{}", xs[0]);
    println!("{:?}", xs);
    println!("{:#?}", xs);

    let refered_xs = &xs[2..4]; // & sign is the refer type which refers to xs array
    println!("{:?}", refered_xs);
}
#[allow(dead_code)]
fn string_demo() {
    /*
    ** @dev String type in rust is just a compound version of 'char'
    */
    let s = "String"; // This will be an &str type not really a str type // or we can use "String".to_string()
    println!("{}", s);
    let real_s = String::from("String!!"); // This become a str type
    println!("{}", real_s);

    let slice_real_s = &real_s[2..4];
    println!("{:?}", slice_real_s);

    let s2 = "String2";
    let c = s.to_string() + s2; // str type + &str type
    println!("{}", c);
    
}


/*
** @dev Ownership section
*/
#[allow(dead_code)]
fn move_ownership(v: Vec<i32>) {
    println!("Moving ownership of {}", v[10] + v[100])
}
#[allow(dead_code)]
fn copy_ownership(a: i32, b: i32) {
    println!("{}", a * b)
}
#[allow(dead_code)]
fn return_ownership(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[120] + v[111]);
    v
}
#[allow(dead_code)]
fn borrow_ownership1(v: &Vec<i32>) {
    println!("{}", (*v)[10] + (*v)[12])
}
#[allow(dead_code)]
fn borrow_ownership2(v: &Vec<i32>) {
    println!("{}", v[10] + v[13])
}

#[allow(dead_code)]
fn ownership() {
    
    let mut v = Vec::new();

    for i in 1..1000 {
        v.push(i);
    }

    // move_ownership(v);
    println!("HEAP datatype moved");

    /**
    ** value moved here (Vec is a complicated type, it is stored in heap not stake.)
    ** It is so that the ownership is moved and is no longer be able to used.
    ** println!("{}", v[0]) // --> this will show error "borrow of moved value: v"
    */
    let a = 32;
    let b = 45;
    copy_ownership(a, b);
    println!("We still have a: {} and b: {} in this scope because int is stored in stake", a, b);

    /*
    ** Borrowing is a reference (similar as pointer in other languages: go or c)
    ** A reference is also a object
    ** Mutable references are moved
    ** Imutable reference are copied
    ** When the reference is dropped and the borrow ends (reference disappears from the scope) --> Another concept is lifetime
    */
    v = return_ownership(v);
    borrow_ownership1(&v);
    println!("Still own v: {} {}", v[0], v[1]);
    borrow_ownership2(&v);
    println!("Still own v: {} {}", v[2], v[3]);

}


use core::panic;
/*
** struct / naming space / method(impl) / Derive Debug trait/ impl custom trait
*/
use std::fmt::{self, Result};
#[derive(Debug)]

struct Object {
    width: u32,
    height: u32
}

// methods
impl Object {
    
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show(&self) {
        println!("{} x {} = {}", self.width, self.height, self.area());
    }
    
    
}
// related function (it is not a method). It creates a namespace of the type called Object
impl Object {
    fn new(width: u32, height: u32) -> Object {
        Object { width: width, height: height }
    }
}

// Implement fmt::Disply (create trait)
#[allow(dead_code)]
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} x{}", self.width, self.height)
    }
}

#[allow(dead_code)]
fn struct_method() {
    let o = Object {
        width: 35,
        height: 55
    };
    o.show();

    let obj = Object::new(32, 52);

    obj.show();

    // Derive Debug
    println!("{:#?}", obj);

    println!("{}", obj); // Err: `Object` doesn't implement `std::fmt::Disply --> So we need to impl it.
    
}


/*
** @dev loop/ while/ for/ break/ match
** @annotation while and for has the same: for i in 1..100 {} (exlucsive range) / while i == 0 {}
*/

#[allow(dead_code)]
fn loop_demo() {
    let mut c = 0;
    loop {
        println!("iter to {}", c);

        c += 1;

        if c > 5 {
            break;
        }        
    }
}

#[allow(dead_code)]
fn loop2_demo() {
    'a: loop {
        println!("loop a");
        'b: loop {
            println!("loop b");
            'c: loop {
                println!("loop c");
                break 'b;
            }
        }
    }
}

#[allow(dead_code)]
fn match_demo() {
    let x = 5;
    match x {
        1 => println!("One"),
        2 => println!("TWO"),
        3 => println!("THREE"),
        4 => println!("FOUR"),
        5 => println!("FIVE"),
        _ => println!("OTHERS"),
    }
}

#[allow(dead_code)]
fn match2_demo() {
    let x = 11;
    match x {
        2 => println!("TWO"),
        1| 3| 5| 7| 9 => println!("Odd number"),
        10..=15 => println!("10 - 15"),
        _ => println!("OTHERS")
    }
}

#[allow(dead_code)]
// binding varaible in tuple
fn match3_demo() {
    let pair = (0, 5);
    match pair {
        (0, y) => println!("Y : {}", y),
        (x,0) => println!("X : {}", x),
        _ => println!("OTHERS")
    }
}

#[allow(dead_code)]
// Add if statement
fn match4_demo() {
    let pair = (5, -5);
    match pair {
        (x, y) if x + y == 0 => println!("= 0"),
        (x, y) if x == y => println!(" = "),
        (x, _) if x % 2 == 0 => println!("EVEN"),
        _ => println!("OTHERS")
    }
}

// binding variable in matach
#[allow(dead_code)]
fn match5_demo() {
    let p = 11;
    match p {
        n @ 1..=10 => println!(" {} < 10 ", n),
        n @ 11..=20 => println!(" {} > 10 & < 20", n),
        _ => println!("OTHERS")
    }

    let p1 = 15;
    let result = match p1 {
        n @ 1..=10 => n,
        n @ 11..=20 => n,
        _ => 0,
    };
    println!("Matching result: {}", result)
}


/*
** @dev Enum & Option
** 
*/


#[derive(Debug)]
enum Direction {
    Up (Point),
    Down (Point),
    Left (Point),
    Right (Point),
}
impl Direction {
    fn match_direction(&self) -> Keys {
        match self{
            Direction::Up(_) => Keys::UpKey(String::from("Press w")),
            Direction::Down(_) => Keys::DownKey(String::from("Press s")),
            Direction::Left(_) => Keys::LeftKey(String::from("Press a")),
            Direction::Right(_) => Keys::RightKey(String::from("Press d")),
            _ => Keys::None(String::from("Dont press key"))
        }
    }
}



#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug)]
enum Keys {
    UpKey (String),
    DownKey (String),
    LeftKey (String),
    RightKey (String),
    None (String)
}


// Create enum type
#[allow(dead_code)]
fn enum_demo() {
    let u = Direction::Up(Point{x: 0, y: 1});
    println!("{:#?}", u);
}

fn enum_demo2() {
    let u = Direction::Down(Point{x: 0, y: -1});
    let key_type = u.match_direction();
    println!("{:#?}", key_type);
}


// enum Option<T> {
//     Some(T),
//     None,
// }

fn option_demo_division(x: f64, y: f64) -> f64 {
    
    let result = if y == 0.0 {
        None
    } else {
        Some(x/y)
    };

    match result {
        Some(res) => res,
        // None => 0.0
        _ => 0.0
    }
}


/*
** @dev Vector / HashMap
** 
*/


fn vecter_demo() {
    
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    for i in &v {
        println!("{}", i);
    }
    
    v.push(9);
    
    println!("body: {:?}, length: {}, capablility: {} ", &v, v.len(), v.capacity());

    println!("{:?}", v.pop());

    println!("body: {:?}, length: {}, capablility: {} ", &v, v.len(), v.capacity());

}

fn vector2_demo() {
    let r = vec![
        MultiType::Float(30.1),
        MultiType::Int(30),
        MultiType::Text(String::from("string"))
    ];
    println!("{:?}", r);
}
#[derive(Debug)]
enum MultiType {
    Float(f64),
    Int(i32),
    Text(String)
}

use std::collections::HashMap;
fn hashMap_demo() {

    let mut hashMap = HashMap::new();
    hashMap.insert(String::from("v1"), 12);
    hashMap.insert(String::from("v2"), 13);
    hashMap.remove(&String::from("v2"));
    for (k, v) in &hashMap {
        println!("key: {}, value: {}", k, v);
    }

    match hashMap.get(&String::from("v1")) {
        Some(&n) => println!("got value: {}", n),
        _ => println!("Not found")
    }
}

fn if_let_demo() {
    // Assersion of certain parameter.
    let someValue = Some(30);
    if let Some(value) = someValue {
        println!("value exist: {}", value)
    }
}


// explicit conversion (as keyword)
fn as_demo() {
    let f = 99.4321_f32;
    let u = f as u8;
    let c = u as char;
    println!("{}, {}, {}", f, u, c);
    println!("{}", 255 as char);
}

/*
** Result Enum is usually used for error checking in Rust
** - Result allows to see the functions or whatever it is faild. 
** - Rather than geting Some with value, we get OK with value.
** - If fail, we get ERR with error msg 
*/
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

use std::fs::File;
fn result_demo() {
    let f = File::open("text.txt");

    let g = match f {
        Ok(file) => file,
        Err(err) => {
            panic!("{}", err)
        },
    };
}



mod trait_generic;
use crate::trait_generic::{trait_demo, trait2_demo, trait3_demo, generic_demo, generic2_demo};

mod smart_pointer_closure;
use crate::smart_pointer_closure::{box_demo, cons_demo, closure_demo, closure2_demo, closure4_demo, closure5_demo};
use crate::error_handle::{error_handle4};


mod error_handle;


fn main() {
    
    // tuple_demo();
    // array_demo();
    // string_demo();
    // ownership();
    // struct_method();
    // loop_demo();
    // loop2_demo();
    // match_demo();
    // match2_demo();
    // match3_demo();
    // match4_demo();
    // match5_demo();
    // enum_demo();
    // enum_demo2();
    // let res = option_demo_division(2.0, 3.0);
    // println!("{}", res)

    // vecter_demo();
    // vector2_demo();
    // hashMap_demo();
    // if_let_demo();
    // as_demo();
    // result_demo();
    // trait_demo();
    // trait2_demo();
    // trait3_demo();
    // generic_demo();
    // generic2_demo();
    // closure_demo();
    // closure2_demo();
    // closure4_demo();
    // closure5_demo();
    error_handle4()

}

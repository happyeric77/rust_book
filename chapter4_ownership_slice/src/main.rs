/** Ownership 3 rules
 * 1. Each value in Rust has a variable that’s called its owner.
 * 2. There can only be one owner at a time.
 * 3. When the owner goes out of scope, the value will be dropped.
 */

/**
 * 
 * 1. We can define a "SCOPE" by closure: will see the variable no loger avable out of closure --> Ref1
 * 2. Ownership "MOVE": --> Ref2
 *      - When a variable is default stored in "stake" such as string literal(&str), number and char , rust adapts copy mechanism.
 *      - When a varible is stored in "heap" such as string, vector rust adapts MOVE mechanism. But if we still somehow want to use expensive copy in heap, we can call .clone method
 * 3. Ownership "BORROWING"(Reference) --> Ref3
 *      - Moving ownership makes funtion operation very tedious. So rust introduct "&" ownership "BORROW" a reference --> Ref3.1 & Ref3.2
 * 4. Mutable Reference: --> Ref4
 *      - Reference is immutable in default, we can add &mut to make it mutable reference 
 *      - The rules of mutible reference: 
 *          * only 1 mutible varible for a certian data in a particular scope
 *          * we cannot have mutible mutable reference in one scope
 *          * we cannot mix immutable reference and mutable reference in one scope
 * 5. Slice type: Ref --> Ref5
 *      - To get contiguous sequence of elements wthin a collection (see chapter 8)
 *      - A slice is a kind of reference, so it does not have ownership.
 *      - Tye type is call &str
 *      - string literal is also a slice type(&str) as string literal is actually a reference of string location of the binary
 * 6. Slice as mentioned is just a reference to a collection, so it can be used for not only string but also vec<T> as &[T] --> Ref6
 */

fn main() {    
    
    // Ref1: Define a "string literal"(fixed size storing in the binary[stake]) and a "stirng"(dynamic size storing in heap)
    {
        let stringLiteral: &str = "hello";
        let string: String = String::from("hello");
    }

    // Ref2 --> MOVE OWERSHIP
    {
        // Copy
        let x: i32 = 3;
        let y = x;
        print!("{} {}", x, y );

        //Move
        let xx: String = String::from("Hello");
        let yy = xx;
        print!("{}",  yy); // --> Now we can only print yy because xx's ownership is moved to yy. 
        let zz = yy.clone(); 
        print!("{} {}", yy, zz) // --> Use clone method to do COPY instead of MOVE. Then we can print both.
    }

    //Ref3 --> BORROW (Reference) OWNERSHIP
    // 3.1 ISSUE
    {
        let x: String = String::from("Hello");
        let x_len: usize = return_length(x);
        
        fn return_length(x: String) -> usize {
            x.len()
        }
        print!("{}'s length is {}", "x", x_len)
    }
    // 3.2 Solution
    {
        let x: String = String::from("Hello");
        let x_len: usize = return_length(&x);
        
        fn return_length(x: &String) -> usize {
            x.len()
        }
        print!("{}'s length is {}", x, x_len) // 
    }

    // Ref4
    {
        let mut x: String = String::from("Hello");          // make x mutable by "mut" keyword
        let x_len_plus_one: usize = return_length_puls_one(&mut x);  // pass mutable x into function by "&mut" keyword
        
        fn return_length_puls_one(x: &mut String) -> usize {
            x.len() + 1
        }
        print!("{}'s length is {}", x, x_len_plus_one)
    }

    // Ref5
    {
        let x: String = String::from("Hello world!");   // String stored in heap
        let xx: &str  = &x[..];                         // String slice reference to a heap location of String
        let y: &str = "Hello ColorfulLife!";
        
        print!("{} {}", first_word(xx), first_word(y));
        fn first_word(val: &str) -> &str {
            let converted = val.as_bytes();
            for (i, &item) in converted.iter().enumerate() {
                if item == b' ' {
                    return &val[0..i];
                }
            }
            &val[..]
        }
    }
    // Ref6
    {
        let mut x: Vec<i32> = Vec::new();
        x.push(30);
        x.push(50);
        let xx: &[i32] = &x[..];
    }
    
}

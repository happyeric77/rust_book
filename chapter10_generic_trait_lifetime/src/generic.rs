/**
 * Generic is to let the function accept multiple type of input args.
 *      1. Create: put <T> followed by function body --> Ref1
 *      2. Create in impl & struct (or Enum): put<T> followed by impl ; put <T> followed by struct body --Ref2
 *  In this point, we will see that even we can bring different type into a function scope, we cannot do any operation for it because rust compiler does not know how to interact with each other between different type.
 *  For example, rust can not be sure if type T is capable to do "+" when we try to operate x + y
 *  This is where Trait comes into play. Ref3
 */


pub fn main() {

    // Ref1: function to conver two variable to a tuple but do not want to constraint its type
    {
        fn consle_xny<T, U>(x: T, y: U) -> (T, U) {
            (x, y)
        }
        let xny = consle_xny(1, 1.0);
        print!("(x, y): {:?}\n", xny)
    }


    // Ref2
    {
        struct Xny<T, U> {
            x: T,
            y: U,
        }
        
        impl<T, U> Xny<T, U> {
            fn consle_xnynwnz<W, Z>(self, w: W, z: Z ) -> (T, U, W, Z) {
                (self.x, self.y, w, z)
            }
        }

        let xy = Xny{x: 10, y: 1.0};
        let xywz = xy.consle_xnynwnz('c', "str");
        print!("(x, y, w, z): {:?}\n", xywz)
    }

    //Ref3
    //3.1 basic
    {
        fn x_greater_than_y<T: PartialOrd>(x: T, y: T) -> bool {
            x > y
        }
        print!("x(2) > y(1) = {:?}\n", x_greater_than_y(2, 1))
    }
    //3.2 advanced
    {
        use std::ops::{Add};
        fn x_plus_y<T: Add>(x: T, y: T) -> <T as Add>::Output {
            x + y
        }
        let res = x_plus_y(1, 1);
        print!("x + y = {:?}\n", res)
    }
}
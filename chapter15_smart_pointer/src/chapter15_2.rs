/** # Deref trait enforce pointer as regular reference
 *      1. Normal reference implements deref trait: Ref 1
 *      2. Box is a struct implements deref trait: Ref 2
 *      3. Automatic deref Coercions: convet a reference of one type to a reference of different type: Ref3
 *          - Rust compiler auto calls the deref for &MyBox struct since it implements deref trait: &MyBox<String> --> &String
 *          - String is also a kind of smart point implements deref trait, so rust compiler calls the deref for &String: &String --> &str
 *          - To conclude, &MyBox<String> --> &String --> &str. If rust compiler does not do that automaticly, we will need to do like Ref4
 *      4. Deref Coercion Interacts with Mutability
 *         Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.
 *         Rust does deref coercion when it finds types and trait implementations in three cases:
 *          - From &T to &U when T: Deref<Target=U>
 *          - From &mut T to &mut U when T: DerefMut<Target=U>
 *          - From &mut T to &U when T: Deref<Target=U>
 *  # Drop trait let the smart pointer auto clean the memory after the variable gets out of scope --> Ref5
 *      - We cannot call .drop() method for those struct which implement drop if you want to release the memory before it goes out of scope. 
 *      - Instead, We need to call drop(object) which provided by standard library.
 */


pub fn main() {
                                                // Ref 1
    {
        let v = 5;
        let rv = &v;
        assert_eq!(5, v);
        assert_eq!(5,*rv);
    }
    
    {
        use std::ops::Deref;
                                                // Ref 2
        struct MyBox<T> (T);
        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }
        impl<T> Deref for MyBox<T> {
            type Target = T;
            fn deref(&self) -> &T {
                &self.0
            }
        }

        let v = 6;
        let bv = MyBox::new(v);
        assert_eq!(6, v);
        assert_eq!(6, *bv);                 // In this case, the "*bv" is interchangable with *(bv.deref())

                                            // Ref3
        fn hello(hello: &str) {             // it takes string literal (&str)
            print!("{}", hello)
        }
        let helloBox = MyBox(String::from("Hello colorfulLife"));

        hello(&helloBox);                   // we pass reference of MyBox (&MyBox), But it work
        
                                            // Ref4
        hello(&(&helloBox.deref())[..]);    // or
        hello(&(*helloBox)[..])
    }
                                            // Ref5
    {
        #[derive(Debug)]
        struct MyNewBox(String);
        impl Drop for MyNewBox {      
            fn drop(&mut self) {            // drop motheds takes "mutable" reference since it will be eventually dropped
                println!("{:?} is being dropped", &self)
            }
        }
        let newBoxWithDrop = MyNewBox(String::from("New Box from colorfulLife"));        
    }

}
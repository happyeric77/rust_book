

/**
 * # Reference Counting (Rc): Used when multiple variable refer to the same source. And we do not know which one will use it last before compilation.
 *      1. In Ref1, We see that we are not able to use multi-reference with one Box
 *      2. In Ref2, We use "Reference Cell (Rc)" to accomplish it. --> Rc::clone method is just to increase the reference count.
 *      3. Reference Count can only be used for immutable data. So we are not able to mutate value. What if we want to mutate it. this is where the RefCell comes to play.
 * # Ref Cell (RefCell): Mutating a value inside an immutable value is called the interior mutability patter. Ref3
 * # Summary comparing Box, Rc (reference count) and RefCell
 *      - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
 *      - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
 *      - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
 * # We can use Rc & RefCell when we want the mut reference has multi-owner by wrapping RefCell into Rc, However, it might cause memory leak. Ref4
 * # To avoid the bidirectional owning each other causing memory leak, we need to make use of Weak type. Weak is a version of Rc that holds a non-owning reference to the managed allowcation.
 *      - Calling Weak.upgrade() by Weak returns Option<Rc> --> Since the weak smart pointer does not know if the value has already been dropped or not.
 *      - Calling Rc::downgrade(&Rc) returns Weak 
 */

pub fn main() {
    // Ref 1
    {
        enum List {
            Cons(i32, Box<List>),
            Nil
        }

        // let val = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

    use List::{Cons, Nil};

    let val = Cons(3, Box::new(Cons(2, Box::new(Cons(1, Box::new(Nil))))));

    let a = Cons(5, Box::new(Cons(4, Box::new(val))));
    /* The second reference to val will not be working since val has already moved to a
    
    let b = Cons(7, Box::new(Cons(6, Box::new(val))));
    */
    }
    
    // Ref 2
    {
        use std::rc::Rc;

        enum List {
            Cons(i32, Rc<List>),
            Nil
        }

        use List::{Cons, Nil};

        let val = Rc::new(Cons(3, Rc::new(Cons(2, Rc::new(Cons(1, Rc::new(Nil)))))));

        let a = Cons(5, Rc::new(Cons(4, Rc::clone(&val))));                             // Important to use Rc::clone to just increase the reference count
        println!("The original reference count of value: {:?}", Rc::strong_count(&val));
        
        let b = Cons(7, Rc::new(Cons(6, Rc::clone(&val))));
        println!("The reference count of value increases one to: {:?}", Rc::strong_count(&val));
    }
    // Ref 3
    {
        // Problem:
        let mut a = 32;             // a is defined a mutable var
        let b = &a;                // b is immutable reference of a
        /*                              
        *b = 30;                         // in this case, we are not able to mutate the value when trying to change the underline value by deref b even the underline value a is mutable
        */         
    }
    {
        // Solution:
        use std::cell::RefCell;
        let mut a: RefCell<i32> = RefCell::new(32);             // a is defined a mutable var
        println!("A value is {}", a.borrow());
        let b = &a;                                 // b is immutable reference of a
        *b.borrow_mut() = 5;
        println!("B value is {}", *b.borrow());
    }

    // Ref 4
    {
        use std::rc::Rc;
        use std::cell::RefCell;
        enum List {
            Cons(i32, Rc<RefCell<List>>),
            Nil
        }

        use List::{Cons, Nil};

        let a = Rc::new(RefCell::new(       // Create a as new List 
            Cons(2, Rc::new(RefCell::new(
                Cons(1, Rc::new(RefCell::new(Nil))))
            ))
        ));
        println!("The reference count of a is '{}", Rc::strong_count(&a));

        let b = Rc::new(RefCell::new(       // Create b adding one Cons on top of the reference of a
            Cons(3, Rc::clone(&a))
        ));

        println!("The reference count of a changed to '{}", Rc::strong_count(&a));
        println!("The reference count of b is '{}", Rc::strong_count(&b));

        *a.borrow_mut() = Cons(4, Rc::clone(&b));     // Mutate the underneath value of a to be reference of b  
        println!("The reference count of b changed to'{}", Rc::strong_count(&b));
    }
    // Ref 5
    {         
                                                            // Problem: Server is able to access edge, edge is not able to access server.  
                                                            // However, If creating the parent reference might cause the memory leak as mentioned on Ref4                                           
        use std::cell::RefCell;
        use std::rc::Rc;
        struct Node {
            value: i32,
            children: RefCell<Vec<Rc<Node>>>
        }

        let edge = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![])
        });

        let server = Rc::new(Node {
            value: 10,
            children: RefCell::new(vec![Rc::clone(&edge)])
        });
    }

    {         
                                                            // Solution: Use Weak reference as the parents which will not have the ownership of certian varable.
                                                            
        use std::cell::RefCell;
        use std::rc::{Rc, Weak};
        struct Node {
            value: i32,
            parents: RefCell<Vec<Weak<Node>>>,              // --> Add new parents field with Weak Rc RefCell
            children: RefCell<Vec<Rc<Node>>>
        }

        let edge = Rc::new(Node {                   // Create a new edge with empty parents.
            value: 5,
            parents: RefCell::new(vec![]),                  
            children: RefCell::new(vec![])
        });

        println!("Strong reference count of edge is '{}'", Rc::strong_count(&edge));
        println!("Weak reference count of edge is '{}'", Rc::weak_count(&edge));

        {                                                   // Create a new scope with a server and make it as parent of edge
            let server = Rc::new(Node {
                value: 10,
                parents: RefCell::new(vec![]),
                children: RefCell::new(vec![Rc::clone(&edge)])
            });
            println!("Strong reference count of server is '{}'", Rc::strong_count(&server));
            println!("Weak reference count of server is '{}'", Rc::weak_count(&server));

            *edge.parents.borrow_mut() = vec![Rc::downgrade(&server)];
            
            println!("Strong reference count of edge changed to '{}'", Rc::strong_count(&edge));
            println!("Weak reference count of edge keeps the same as '{}'", Rc::weak_count(&edge));

            println!("Strong reference count of server keeps the same as '{}'", Rc::strong_count(&server));
            println!("Weak reference count of server changed to '{}'", Rc::weak_count(&server));
        }
                                                            // Once the server goes out of scope, it wont cause the issue of memory leak
        println!("Strong reference count of edge changed to '{}'", Rc::strong_count(&edge));
        println!("Weak reference count of edge keeps the same as '{}'", Rc::weak_count(&edge));

    }
}
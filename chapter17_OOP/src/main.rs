/**
 * 1. Rust comes from functional programing language, but it is able to accomplish OOP by different approaches:
 *      - class: --> Struct or Enum
 *      - encapsulation: --> pub keyword
 *      - inherit: use Trait bound with generic. MORE IMPORTANTLY, rust provides "Trait Object" which uses dynamic dispatch indead of static dispatch as generic uses
 * 2. Trait Object makes polymorphism possible in rust. Ref 1
 * 3. Static dispatch vs Dynamic dispatch
 *      - Static dispatch: all possible trait functions will be redefined during compile time.
 *      - Dynamic dispatch: The object type impl the trait is defined during run time.
 * 4. Object safty: We can only make object safty trait into trait bound. If the trait does not have following two properites, then compiler will not be able to figure out the concrete type of that trait.
 *      - The return type is not self
 *      - there is not generic parameter
 */


fn main() {

                                                                        // Ref 1
    {                                                                   // ISSUE: In this case, screen only allowed to contain one type of data which is generic <T>
                                                                        //        However, the fact is that There should be many different type of compoents in the screen such as Header, Container, Footer ..etc.
        trait Draw {                                                    //        It is where the dynamic dispatch (Trait Object) comes into play  
            fn draw(&self);
        }
    
        struct Screen<T: Draw> {
            components: Vec<T>
        }
    
        impl<T: Draw> Draw for Screen<T> {
            fn draw(&self) {
                for compoent in self.components.iter(){
                    compoent.draw()
                }
            }
        }
    }

    {                                                                   // SOLUTION: 
        trait Draw {                                                    // We do not define generic T
            fn draw(&self);
        }

        struct Screen {
            components: Vec<Box<dyn Draw>>                              // We only wrap dynamic dispatch into Box smart pointer inside the vector
        }                                                               // Then the Inside the vector, it allows to be different type which impl the Draw trait
        
        impl Draw for Screen {
            fn draw(&self) {
                for compoent in self.components.iter(){
                    compoent.draw()
                }
            }
        }
    }

    
}

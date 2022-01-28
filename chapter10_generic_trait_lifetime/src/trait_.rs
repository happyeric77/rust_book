/**
 * Trait defines the share behvior accross different types
 *      1. Create trait using trait keyword trait<T> {} // Ref1
 *      2. Dunctions in trait do not need to have function body, which can be implemented freely by custom defined typs --> //Ref2
 *      3. Default implementation: we can define default implementation by adding function body in trait function. In this case, it will adapt default behavior.
 *      4. Trait bound: is to constraint the input argument which implement certain trait
 *          - we can simple use fn some_function(x: &impl trait). It is the short of below
 *          - fn some_function<T: trait>(x: T). We can even make it cleaner by using where keyword Ref 3
 *      5. Constraint for return value to impelement certain trait Ref4
 *      6. Conditional trait impl: We can use trait to do conditional impl. Ex, all objects can have genernal implementation, only those objects with certian trait have special functions.
 */

pub fn main() {
    

    {
        // Ref1
        trait NumberOpts<T: PartialEq + PartialOrd> {
            fn greater_than(&self, x: T);
            fn less_than(&self, x: T);
            fn defulat_implement(&self) {
                print!("This is default impelemnt\n")
            }
        }

        // Ref2
        
        struct MockNum<T: PartialEq + PartialOrd> {
            value: T
        }

        impl<T: PartialEq + PartialOrd> NumberOpts<T> for MockNum<T>{

            fn greater_than(&self, x: T) {print!("{:#?}\n", self.value > x)}
            fn less_than(&self, x: T) {print!("{:#?}\n", self.value < x)}
        }

        let num = MockNum {value: 1};
        num.greater_than(0);
        num.defulat_implement();

        // Ref3
        fn take_mock_num<T, U>(x: T) 
            where   U: PartialEq + PartialOrd, 
                    T: NumberOpts<U> {
            x.defulat_implement()
        }
        take_mock_num(num);

        // Ref4
        fn return_mock_num<T: PartialEq + PartialOrd>(x: T) -> impl NumberOpts<T> {
            MockNum {value: x}
        }
        return_mock_num(1);
    }

    {
        struct MockNum<T> {
            value: T
        }
        impl<T> MockNum<T> {
            fn new(x: T) -> Self {
                Self{value: x}
            }
        }
        trait NumberOpts<T: PartialEq + PartialOrd> {
            fn greater_than(&self, x: T);
            fn less_than(&self, x: T);
            fn defulat_implement(&self) {
                print!("This is default impelemnt\n")
            }
        }
    }
}
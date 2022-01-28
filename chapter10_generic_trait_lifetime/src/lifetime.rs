/** Purpose if lifetime is to prevent invalid reference happening(Dangling Reference)
 *      BORROW CHECKER & DANGLING REFERENCE: 
 *  1. rust's borrow checker checks if all reference value is dangling reference. Ref1
 *  2. ISSUE: In the case of Ref1, BORROW CHECKER can understand "b" will be out of scope after the clousure is exectued. what if the variables are from out side of scope, say a function with arguments out of the scope.
 *  3. 3 rules of default lifetime checker by compiler
 *      - Each parameter that is a reference gets its own lifetime parameter.
 *      - If there is a only lifetime parameter, that lifetime is assgined to all output lifetime parameters
 *      - If there are multiple input lifetime parameter, and one of them is &self or &mut self. The lifetime of "self" is assgined to all output lifetime parameters. 
 *  4. Case 1 (Ref2): Meet rule#1 & rule#2 --> No need generic lifetime annotation
 *  5. Case 2 (Ref3): Meet only rule#3 --> No need generic lifetime annotation
 *  6. Case 3 (Ref4): None of rule can be adapted --> We need to explicitly define lifetime through lifetime annotation.
 *  7. See Ref5 to see summarized example.
 */


pub fn main() {
    // Ref1
    {
        let mut a;
        {
            let b = 3;
            a = &b
        }
        // Do sth with a (dangling reference). ex: print!("{}", a) --> Then fail
    }
    // Ref2
    {
        let a = 3;
        fn function_closure(a: &i32) -> &i32 {
            a
        }
    }
    // Ref3
    {
        struct a {
            val: i32,
        }
        impl a {
            fn aFunction(&self, x: & i32, y: &i32) -> &i32 { // In this case we are not allowed to return x neither y as currently the lifetime is set the same as &self. If returning x or y is needed, go for Ref4
                &self.val
            }
        }
    }
    // Ref4
    {
        let a = 3;
        let b = 4;
        fn a_or_b<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
            if (x > y) {
                x
            } else {
                y
            }
        } 
    }
    // Ref5
    {
        fn a_or_b<'a, T: PartialOrd>(x: &'a T, y: &'a T) -> &'a T {
            if (x > y) {
                x
            } else {
                y
            }
        }
    }
}
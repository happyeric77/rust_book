/**
 * Iterator enforce all a series of data to iterate through.
 * 1. To turn the data type to iterator, use .iter() method --> Ref1
 * 2. Iterator is a trait with next and some others methods impls (zip, map, filter ...etc). As long as a data type implements the iterator trait, we can make use of those methods
 * 3. Iterator trait consumer functions: filter, map, zip, sum --> Ref3
 * 4. We can use iterator to rewrite the chapter12 search mothed. --> Ref4
 *      - The performance between loop and iterator does not make much different. So it turns out selecting either of them according to developers' preference.
 *      - However, Rust developer tend to use iterator since it is higher level of abstraction and we get those nice feature from iterator 
 */


fn main() {
    // Ref1
    {
        let v = vec![1, 2, 3];
        let iterator = v.iter();
        for value in v {
            println!("{}", value)
        }
    }
    
    // Ref2
    trait Iterator {
        type Item;                                      // The associated type which will be introduced more in chapter 19
        
        fn next(&mut self) -> Option<Self::Item>;       // The "&mut" self needs to be input becuase the struct val is changed when calling next

        // Other methods with default implementation.
    }

    // Ref3
    {
        let v1 = vec![1,2,3];
        let v2 = vec![4,5,6];
        let v1_sum: i32 = v1.iter().sum();
        let iterator = v1.iter().zip(v2.iter());                            // Zip two iterator together -> [(a1, b1), (a2, b2), ...]
        let filtered_result = v1.into_iter().filter(|item| item == &1);     // Use filter method with closure to add additional filter logic
                                                                                                        // into_iter meothd takes the ownership of the collection var --> In this case, all elements in v1 are moved.
        let map_result = v2.iter().map(|item| item+1);
        let convertedToCollection: Vec<i32> = filtered_result.collect();
    }

    // Ref4

    fn search_case_sensitive<'a>(query: &String, content: &'a String) -> Vec<&'a str> {     
        let lines = content.lines();
        lines.filter(|line| line.contains(query)).collect()
    }
}

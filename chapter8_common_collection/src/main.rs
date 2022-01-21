/** OVERVIEW
 *  Unlike TUPLE and ARRAY, Common Collection is stored in heap. Meaning the size can grow or shrink.
 *      - Vector 
 *      - String
 *      - HashMap
 *  # Vector:
 *      1. Create: Ref1
 *          - Vector can host any type of value, so we need to define the type inside the vector when creating one --> Ref1
 *          - vec! macro helps to init a new Vector with initial value without explicitly define the type becaseue rust will auto detect the type in the intial value.
 *      2. Access element: Ref2
 *          - Direct access by &[?]: this will potentially cause run time error because vector is allowcated on heap so that the size can grow or shrink during runtime.
 *          - Get method: It returns a option value to help us to handle the the case if the index is not exist
 *      3. Iterate: We can easly use "for i in &vec {}" or "for i in &mut vec{}"
 *      4. We can use enum vector to store various type inside enum.
 *  # String: "collection" of "encoded" utf-8 bytes
 *      1. Create: 
 *          - String::new() to create a empty string
 *          - &str.to_string() to convert string slice to String type
 *          - String::from("??") to create a new String with initial value
 *      2. Append:
 *          - .push_str() which takes a &str because we do not want to take the ownership of that input arg
 *          - .push() which takes char
 *          - + &str or &String: this will take the ownership from original string
 *          - format!("{}{}", s1, s2): format macro will copy two strings and appends it together
 *      3. Access(indexing): Ref3
 *          - three revalent ways a word is represented in unicode: byte, scalar and grapheme
 *  # HashMap: To use HashMap, we need to import standard crate by "use std::Collections::HashMap" --> Ref4
 *      1. Create: HashMap::new() --> Ref4.1
 *      2. Add: .insert(key, value) --> Ref4.2  .insert will take the ownership of passed in var. If it is not what wanted, we can pass reference in with life time defined (chaper 10)
 *      3. Access: .get() --> Ref4.3 it returns an Option
 *      3. Utility methods: 
 *          - .entry().or_insert() --> Ref4.4 it returns an &mut value, we can de-reference(*) it and change the val.
 */

fn main() {
    //Ref1
    {
        let v: Vec<i32> = Vec::new();   // Empty vector (need to use push to add value)
        let vv = vec!([1,2,3,4,5]);
    }
    // Ref2
    {
        let v = vec!([1,2,3,4,5]);
        // let av = &v[10];                // No issue during compilation, but error during runtime.  

        let vv = vec!([6,7,8,9,10]);
        let avv = vv.get(10);           // Return Option::None instead of throwing error during runtime.
    }
    // Ref3 
    {
        let s = String::from("Hello ColorfulLife, 您好彩色人生，こんにちわ");
        // Represented by Bytes
        print!("String in Bytes: ");
        for i in s.bytes(){
            print!("{} ", i);
        }
        print!("\n");
        // Represented by scalar
        print!("String in scalar: ");
        for i in s.chars() {
            print!("{}", i);
        }
        print!("\n");
        // Represented by Grapheme --> need to bring in dependency crate called unicode-segmentation (in Cargo.toml)
        use unicode_segmentation::UnicodeSegmentation;
        print!("String in Graphemes: ");
        for i in s.graphemes(true){
            print!("{}", i);
        }
        print!("\n")
    }
    // Ref4
    {
        //4.1
        use std::collections::HashMap;
        let mut h: HashMap<String, i32> = HashMap::new();
        //4.2
        h.insert(String::from("foo"), 1);
        print!("Ref4.2\nInsert new foo: {:#?}\n", h);
        
        h.insert(String::from("foo"), 2);
        print!("Ref4.3\nReplace old foo to new foo: {:#?}\n", h);
        
        //4.3
        let foo = h.get("foo");
        match foo {
            Some(value) => print!("Ref4.3: use get method\nValue found: {}\n", value),
            None => print!("Ref4.3: use get method\n: No value")
        }

        //4.4
        let s = String::from("Hello ColorfulLife Hello world");
        let mut hh: HashMap<String, i32> = HashMap::new();
        for i in s.split_whitespace() {
            let count = hh.entry(i.to_string()).or_insert(0);
            *count += 1;
        }
        print!("{:#?}", hh)
    }
}

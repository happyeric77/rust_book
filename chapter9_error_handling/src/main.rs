/**
 * 1. Panic macro & RUST_BACKTRACE=1: Use panic!("??") can crash program in runtime. To trace the error history, we can call "RUST_BACKTRACE=1 cargo run"
 * 2. Result Enum: Ref1
 *      - Result is an enum just like Option, but it brings OK() & ERR(). Since it is used very often, it is brough into scope by default. --> Ref1.1
 *      - std::io::ErrorKind --> We can handle error according to different error type --> Ref1.2
 *      - unwrap_or_else: Result enum impl this method which takes an closure (|error|{}). If the value is OK then get the value underneth, else handle the error inside the clouse. --> Ref1.3
 *      - unwrap: If we as developer can be sure the Result has OK value, then we can just use unwrap to get the value
 *      - expect: We can use expect method to get the underneth value. it will return given &str message when the enum has Err value --> Ref1.4
 * 3. Error propergation: handle the err inside a function and return a Result enum. Ref2
 *      - Basic usage by match method Ref2.1
 *      - ? operator to simplify Error propergation Ref2.2: Use ? to let rust justify if Result enum has OK value. 
 *           # Yes: extract the value underneth
 *           # No: reutrn Err(e) as fn return
 *      - Note: fs provides a method called fs::read_to_string(file path) which do the same thing as above
 */


use std::fs::File;

fn main() {
    
    // Ref1
    {
        // 1.1
        let rs: Result<File, std::io::Error> = File::open("hello_colorfulLife_Ref1");
        match &rs {
            Ok(_) => print!("File fetched successfully!!\n"),
            Err(e) => print!("Fail to fetch file: {}\n", e)
        }
        // 1.2
        use std::io::ErrorKind;
        match &rs {
            Ok(_) => print!("File fetched successfully!!\n"),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    let create_rs = File::create("hello_colorfulLife\n");
                    match create_rs {
                        Ok(_) => print!("File created\n"),
                        Err(e) => print!("Creating file failed: {}\n", e)
                    }
                }
                others => print!("Fetching file fails: {:#?}\n", others) 
            }
        }        
    }
    // Ref1.3
    {
        let rs = File::open("./hello_colorfulLife_ref1.3");
        let f = rs.unwrap_or_else(|e|{
            panic!("ERR"); // --> Since unwrap_or_else expect a File type as an output. We either use panic to throw error during runtime or hanle error then output a File object
        });
    }

    {
        let rs = File::open("./hello_colorfulLife_ref1.3");
        let f = rs.expect("Error happened!");
    }

    // Ref2.1
    fn read_file_content_basic() -> Result<String, std::io::Error> {
        let rs = File::open("hello_colorfulLife_Ref2.1");
        // check if file fetched, if not return Error as function return.
        let mut f = match rs {
            Ok(file) => file,
            Err(e) => return Err(e)
        };

        let mut s = String::new();
        // import Read crate when want to read file content to use .read_to_string method
        use std::io::Read;
        let content = f.read_to_string(&mut s);
        match content {
            Ok(_) => return Ok(s),
            Err(e) => return Err(e)
        }
    }

    fn read_file_content_advanced() -> Result<String, std::io::Error> {
        
        // Use ? to let rust justify if Result enum has OK value. 
        //      - if Yes: extract the value underneth
        //      - if No: reutrn Err(e) as fn return
        
        let mut rs = File::open("hello_colorfulLife_Ref2.2")?;
        let mut s = String::new();
        use std::io::Read;
        rs.read_to_string(&mut s)?;
        Ok(s)
    }

}

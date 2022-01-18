
/*
 * 1. Enum in Rust as other language can store serial data ex1. IpAddressKind
 * 2. Enum in Rust can bring extra data in various data type ex2. IpAddress
 * 3. Enum can also impl method ex3. printAddr
 * 4. Option in Rust is a Special built-in type which help to handle none value more efficiently. ex3. Option
 * 5. Error handling through Option enum
 *      - Use if let 
 *      - Use match
 */

#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String)
}

impl IpAddress {
    fn printAddr(&self) {
        print!("\nEx3: impl function with enum: {:?}", self)
    }
}

// Option in Rust is "BUILT-IN", we do not need to write this manually. Below is just for demo purpose
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    // Ex1
    let ipV4 = IpAddressKind::V4;
    let ipV6 = IpAddressKind::V6;
    print!("\nEx1: basic enum: {:#?} & {:?} \n", &ipV4, &ipV6);
    // Ex2
    let ipV4WithAddr = IpAddress::V4(192, 168, 1, 1);
    let ipV6WithAddr = IpAddress::V6(String::from("ifeja:488jfsr:40p9jf:3fjp9q4"));
    print!("Ex2: enum with value: {:?} & {:?} \n", ipV4WithAddr, ipV6WithAddr);
    // Ex3
    ipV4WithAddr.printAddr();
    // Ex4
    let optionValue = Option::Some(7);
    print!("Ex4: Option enum with some value {:?}\n", &optionValue);
    // Ex5.1
    if let optionValue = Option::Some(7) {
        print!("Ex5: Use if let to confirm there is some value in option\n");
    }
    // Ex5.2
    match optionValue {
        Option::Some(7) => print!("Ex5.2 - 1: Use match to handle option enum\n"),
        _ => panic!("Ex5.2 - 2: use underscore to let rust handle all other cases\n")
    }

}

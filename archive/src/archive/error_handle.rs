/*
** Option and Result Enum are the way to hanle error
*/

enum Result<T, E>{
    Ok(T),
    Err(E),
}

enum Option<T> {
    Some(T),
    None,
}

use std::fs::File;

fn error_handle() {
    let f = File::open(".text.txt");
    let f = match f {
        Ok(file) => file,
        Err(e) => {
            panic!("{}", e)
        }
    };
}

fn error_handle2() {
    let f = File::open(".text.txt").unwrap();
    let f1 = File::open(".text.txt").expect("Could not open the file");
}

use std::io::{self, Read};
// use std::fs::Read;

fn error_handle3() -> Result<File, io::Error> {
    let f = File::open(".text.txt");
    let mut f = match f {
        Ok(file) => return Result::Ok(file),
        Err(e) => return Result::Err(e),
    };


    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => s,
    //     Err(e) => Err(e)
    // }
}
pub fn error_handle4() {
    let f = error_handle3();
}

// fn error_handle5() -> Result<File, io::Error> {
//     let mut f = File::open(".text.txt")?;
//     Result::Ok(f)
//     // let mut f = match f {
//     //     Ok(file) => return Result::Ok(file),
//     //     Err(e) => return Result::Err(e),
//     // };


//     // let mut s = String::new();
//     // match f.read_to_string(&mut s) {
//     //     Ok(_) => s,
//     //     Err(e) => Err(e)
//     // }
// }


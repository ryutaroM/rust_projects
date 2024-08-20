use std::fs::File;
use std::io;
use std::io::Read;
fn main() {
    // let v = vec![1, 2, 3];
    // v[99];

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(f) => f,
    //     Err(e) => {
    //         panic!("There is a problem opening the file: {:?}", e)
    //     },
    // };

    // let f = match f {
    //     Ok(f) => f,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
    //         Ok(f) => f,
    //         Err(e) => {
    //             panic!("Tried to create file but there was a problem!: {:?}", e)
    //         }
    //     },
    //     Err(e) => {
    //         panic!("There was a problem opening the file: {:?}", e)
    //     }
    // };

    // let f = File::open("hello.txt").unwrap();

    // let f = File::open("hello.txt").expect("you can choose message");

    match read_username_from_file() {
        Ok(name) => println!("user name is :{}", name),
        Err(e) => println!("failed open file {:?}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(f) => f,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // ?operator
    // let mut f = File::open("hello.tx")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // short
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

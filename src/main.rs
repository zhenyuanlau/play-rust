use std::fs::File;

fn main() {
    // println!("Hello, world!");

    // panic!("crash and burn");

    // let v = vec![1, 2, 3];

    // v[99];

    let f = File::open("hello.txt");

    // let f: u32 = File::open("hello.txt");

    match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}

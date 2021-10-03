use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // let f = File::open("hello.txt").unwrap();

    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    let result_1 = read_username_from_file_1();
    println!("{:?}", result_1);
    let result_2 = read_username_from_file_2();
    println!("{:?}", result_2);
    let result_3 = read_username_from_file_3();
    println!("{:?}", result_3);
    let result_4 = read_username_from_file_4();
    println!("{:?}", result_4);

    let f = File::open("hello.txt")?;

    Ok(())
}

fn read_username_from_file_1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

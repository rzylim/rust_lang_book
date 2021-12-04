use std::fmt;
use std::io::Error;

fn main() {
    type Kilometres = i32;
    let x: i32 = 5;
    let y: Kilometres = 5;
    println!("x + y = {}", x + y);

    let f1: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    fn takes_long_type(f1: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }
    // fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    //     // --snip--
    // }
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f2: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type1(f2: Thunk) {
        // --snip--
    }
    // fn returns_long_type1() -> Thunk {
    //     // --snip--
    // }

    // bar();

    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
}

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    // fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    // fn flush(&mut self) -> Result<(), Error>;
    // fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    // fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;

    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn bar() -> ! {
    print!("forever ");

    loop {
        print!("and ever ");
    }
}

// fn generic<T: ?Sized>(t: &T) {
//     // --snip--
// }

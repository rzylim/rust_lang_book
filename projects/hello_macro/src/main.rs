use hello_macro::HelloMacro;

fn main() {
    Pancakes::hello_macro();

    // let sql = sql!(SELECT * FROM posts WHERE id=1);
}

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

// hello_macro = { path = "../hello_macro" }
// hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers1 = vec![1, 2, 3];
    let list_of_strings1: Vec<String> = list_of_numbers1.iter().map(|i| i.to_string()).collect();
    let list_of_numbers2 = vec![1, 2, 3];
    let list_of_strings2: Vec<String> = list_of_numbers2.iter().map(ToString::to_string).collect();

    let list_of_statuses1: Vec<Status> = (0u32..20).map(Status::Value).collect();
    let list_of_statuses2: Vec<Status> = (0u32..20).map(|i| Status::Value(i)).collect();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    Stop,
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

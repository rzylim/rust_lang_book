use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for item in row {
        println!("{:?}", item)
    }

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{}: {:?}", team_name, score);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let v = vec![1, 2, 3, 4, 5, 2];
    println!("mean: {}", mean(&v));
    println!("median: {}", median(&v));
    println!("mode: {}", mode(&v));
}

fn mean(v: &[isize]) -> f64 {
    let mut sum = 0.0;
    for num in v {
        sum += *num as f64;
    }
    sum / v.len() as f64
}

fn median(v: &[isize]) -> f64 {
    let mut new_v = v.to_owned();
    new_v.sort_unstable();
    println!("{:?}", new_v);
    let length = v.len();
    if length % 2 == 0 {
        (new_v[length / 2 - 1] + new_v[length / 2]) as f64 / 2.0
    } else {
        new_v[(length + 1) / 2] as f64
    }
}

fn mode(v: &[isize]) -> isize {
    let mut counts: HashMap<isize, usize> = HashMap::new();
    for num in v {
        let counter = counts.entry(*num).or_insert(0);
        *counter += 1;
    }

    let mut max: (isize, usize) = (v[0], 0);
    for (num, count) in counts.iter() {
        if *count > max.1 {
            max = (*num, *count);
        }
    }
    max.0
}

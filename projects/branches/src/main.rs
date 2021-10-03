use inflector::Inflector;

fn main() {
    let number = 6;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    println!(
        "Celsius -> Fahrenheit: 0 -> {}",
        convert_temperature(false, 0)
    );
    println!(
        "Fahrenheit -> Celsius: 32 -> {}",
        convert_temperature(true, 32)
    );

    println!("1st Fibonacci number: {}", fib(1));
    println!("5th Fibonacci number: {}", fib(5));
    println!("10th Fibonacci number: {}", fib(10));

    the_twelve_days_of_christmas();
}

// Convert temperatures between Fahrenheit and Celsius.
// converts to fahrenheit if to_celsius is false
fn convert_temperature(to_celsius: bool, temp: i32) -> i32 {
    if to_celsius {
        (temp - 32) * 5 / 9
    } else {
        temp * 9 / 5 + 32
    }
}

// Generate the nth Fibonacci number.
fn fib(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn the_twelve_days_of_christmas() {
    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    const GIFTS: [&str; DAYS.len()] = [
        "a patridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for (verse, n_day) in DAYS.iter().enumerate() {
        println!("On the {} day of Christmas my true love sent to me", n_day);
        if verse == 0 {
            println!("{}.\n", GIFTS[0].to_sentence_case())
        } else {
            for gift in GIFTS.iter().take(verse + 1).skip(1).rev() {
                println!("{},", gift.to_sentence_case())
            }
            println!("And {}.\n", GIFTS[0]);
        }
    }
}

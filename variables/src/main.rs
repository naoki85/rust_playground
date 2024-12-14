// use std::io;

// const MAX_POINTS: u32 = 100_000;

fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    //
    // println!("The MAX POINTS is: {}", MAX_POINTS);
    //
    // let y = 5;
    //
    // let y = y + 1;
    //
    // {
    //     let y = y * 2;
    //     println!("The value of y in the inner scope is: {}", y)
    // }
    //
    // println!("The value of y is: {}", y);
    //
    // let spaces = "   ";
    // let spaces = spaces.len();
    // println!("The value of spaces is: {}", spaces);
    //
    // let x = 2.0; // f64
    // println!("The value of x is: {}", x);
    // let x: f32 = 3.0; // f32
    // println!("The value of x is: {}", x);
    //
    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;
    // println!(
    //     "The each value of x: {}, {}, {}",
    //     five_hundred, six_point_four, one
    // );
    //
    // let a = [1, 2, 3, 4, 5];
    // println!("Please enter an array index.");
    // let mut index = String::new();
    //
    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");
    //
    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");
    //
    // let element = a[index];
    // println!(
    //     "The value of the element at index {} is: {}",
    //     index, element
    // );

    println!("Hello, world!");
    another_function(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = plus_one(5);
    println!("The value of x is: {}", x);

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 3;
    if number != 0 {
        println!("condition was false");
    }

    let number = 6;

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
    println!("The number of value is: {}", number);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut number = 3;

    while number != 3 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!");
}

fn another_function(x: i32, unit_label: char) {
    println!("Another function.");
    println!("The measurement is: {}{}", x, unit_label);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

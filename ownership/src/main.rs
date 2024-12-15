fn main() {
    // let s = String::from("hello");
    // takes_ownership(s);

    // let x = 5;
    // makes_copy(x);
    // println!("The value of x is: {}", x);
    // println!("The value of s is: {}", s);

    // let mut s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);

    // change(&mut s1);
    // println!("The value of s1 is {}.", s1);

    // {
    //     let mut r1 = &mut s1;
    // }
    // let mut r2 = &mut s1;
    // println!("{}, {}", r1, r2);

    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("The value of word is {}", word);

    let my_string_literal = "hello world";
    // let word = first_word(&my_string_literal[..]);
    let word = first_word(my_string_literal);
    println!("The value of word is {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }
//
// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

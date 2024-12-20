pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let news_article = NewsArticle {
        headline: String::from("horse_ebooks"),
        location: String::from("head"),
        author: String::from("naoki85"),
        content: String::from("Hello"),
    };
    println!("1 new article: {}", news_article.summarize());
}

// use std::fmt::Display;

// fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
// where
//     T: Display,
// {
//     println!("Announcement! {}", ann);
//
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

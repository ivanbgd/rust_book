#![allow(unused)]


use std::fmt::Display;
use aggregator_ivanbgd::{NewsArticle, Summary, Tweet};
use aggregator_ivanbgd::{notify, notify2, notify3, notify4, notify5, notify6};
use aggregator_ivanbgd::{some_function1, some_function2};
use aggregator_ivanbgd::{returns_summarizable};
use aggregator_ivanbgd::{Pair};


fn find_max(array: &[i32]) -> Option<i32> {
    if array.len() > 0 {
        let mut max = &array[0];
        for elt in array {
            if elt > max {
                max = elt;
            }
        }
        Some(*max)
    } else {
        None
    }
}

fn largest_i32(array: &[i32]) -> &i32 {
    // No index checking - for conciseness reasons.
    let mut largest = &array[0];
    for elt in array {
        if elt > largest {
            largest = elt;
        }
    }
    largest
}

fn largest_char(array: &[char]) -> &char {
    // No index checking - for conciseness reasons.
    let mut largest = &array[0];
    for elt in array {
        if elt > largest {
            largest = elt;
        }
    }
    largest
}

fn largest<T: std::cmp::PartialOrd>(array: &[T]) -> &T {
    // No index checking - for conciseness reasons.
    let mut largest = &array[0];
    for elt in array {
        if elt > largest {
            largest = elt;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

enum MyOption<T> {
    Some(T),
    None,
}

enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/// The type Point<f32> will have a distance_from_origin method;
/// other instances of Point<T> where T is not of type f32 will not have this method defined.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct PointMixed<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointMixed<X1, Y1> {
    /// A method that uses generic types different from its struct’s definition
    fn mixup<X2, Y2>(self, other: PointMixed<X2, Y2>) -> PointMixed<X1, Y2> {
        PointMixed {
            x: self.x,
            y: other.y,
        }
    }
}

fn longest_string<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.len() >= string2.len() {
        string1
    } else {
        string2
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("Announcement! {}", ann);
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement2<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let array = vec![];
    let max = match find_max(&array) {
        Some(max) => max,
        None => -34,
    };
    println!("max = {}", max);
    assert_eq!(max, -34);
    println!("max = {}", match find_max(&array) {
        Some(max) => max,
        None => -34,
    });
    match find_max(&array) {
        Some(max) => println!("max = {}", max),
        None => println!("Empty array!"),
    };
    // println!("max = {}", find_max(&array).expect("Got empty array!"));

    let array = vec![34, 50, 25, 100, 65];
    println!("max = {}", find_max(&array).expect("Got empty array!"));

    let array = [34, 50, 25, 100, 65];
    println!("max = {}", find_max(&array).expect("Got empty array!"));

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}.", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}.", result);

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 1, y: -2 };
    let float = Point { x: -3.4, y: 5.6 };
    println!("integer = {:?}\nfloat = {:?}", integer, float);

    // They can be of the same type, too.
    let mixed_point = MixedPoint { x: -3, y: 5.6 };
    println!("mixed point = {:?}", mixed_point);  // mixed point = MixedPoint { x: -3, y: 5.6 }
    let mixed_point = MixedPoint { x: 'd', y: "aBc" };
    println!("mixed point = {:?}", mixed_point);  // mixed point = MixedPoint { x: 'd', y: "aBc" }

    let p = Point { x: 2, y: 4 };
    println!("p.x() = {}", p.x());  // p.x() = 2

    let p = Point { x: 2., y: 4. };
    println!("p.distance_from_origin() = {}", p.distance_from_origin());  // p.distance_from_origin() = 4.472136

    let point_mixed = PointMixed { x: -3, y: 'd' };
    println!("point mixed = {:?}", point_mixed);  // point mixed = PointMixed { x: -3, y: 'd' }

    let p1 = PointMixed { x: 3, y: 43.43 };
    let p2 = PointMixed { x: "ABC", y: 'D' };
    let p3 = p1.mixup(p2);
    println!("point mixed p3 = {:?}", p3);  // point mixed p3 = PointMixed { x: 3, y: 'D' }

    println!("\n");
    let tweet = Tweet {
        username: String::from("my_username"),
        content: String::from(
            "Demo content",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("\nTweet:\n{}", tweet);

    println!("\n");
    let vec = vec!(4, -3, 2);
    println!("vec = {:?}; len = {}\n", vec, vec.len());
    println!("vec.summarize():\n{}", vec.summarize());

    println!("\n");
    let article = NewsArticle {
        headline: String::from("A team won a championship!"),
        location: String::from("Europe"),
        author: String::from("Name Lastname"),
        content: String::from(
            "The team once again are the best team in the league.",
        ),
    };
    println!("New article available! {}", article.summarize());

    println!("\n");
    notify(&vec);
    notify(&tweet);
    notify(&article);
    notify2(&article);
    println!("\n");
    notify3(&tweet, &vec);
    println!("\n");
    notify4(&tweet, &tweet);
    println!("\n");
    notify5(&tweet);
    println!("\n");
    notify6(&tweet);

    println!("\n");
    println!("{}", some_function1(&5, &7));
    println!("{}", some_function2(&5, &-7));

    println!("\n");
    let returned_tweet = returns_summarizable();
    // println!("The returned default tweet:\n{}", returned_tweet);

    println!("\n");
    let pair = Pair::new(2, 3);
    println!("Pair = {}", pair);
    println!("Pair = {:?}", pair);
    println!("Pair = {:#?}", pair);
    pair.cmp_and_display();
    Pair::cmp_and_display(&pair);

    println!("\n");
    println!("{}", pair.to_string());  // Pair implements Display, so this is possible to do.
    println!("{}", 33.2.to_string());

    println!("\n");
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest_string(string1.as_str(), string2);
    println!("The longest string is: {}", result);

    let string1 = String::from("a long string");
    let result2;
    {
        let string2 = String::from("wxyz");
        result2 = longest_string(string1.as_str(), string2.as_str());
        println!("The longest string is: {}", result2);
    }

    println!("\n");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'.");
    let important_excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("important_excerpt: {:?}", important_excerpt);
    println!("important_excerpt.level(): {}", important_excerpt.level());
    println!("important_excerpt.announce_and_return_part(): {}",
             important_excerpt.announce_and_return_part("Hi!"));

    println!("\n");
    let s1: &str = "I have a static lifetime.";
    let s2: &'static str = "I have a static lifetime, too.";
    println!("s1: {}\ns2: {}", s1, s2);

    println!("\n");
    let result = longest_with_an_announcement("abc", "1234", "Birthday!");
    println!("The longest string is: {}", result);
}

// #![allow(dead_code)]
#![allow(unused)]
#![allow(unused_doc_comments)]

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::io::stdin;


/// https://doc.rust-lang.org/std/collections/index.html
fn documentation_examples() {
    let vec = vec![1, 2, 3, 4];
    for x in vec.iter() {
        println!("vec contains: {x:?}");
    }
    for x in vec {
        print!("vec contains: {x} ");
    }
    println!("\n");

    let mut vec = vec![1, 2, 3, 4];
    for x in vec.iter_mut() {
        *x += 1;
        print!("{x} ");
    }
    println!("\n{:?}", vec);
    println!();
    let mut vec = vec![1, 2, 3, 4];
    for x in &mut vec {
        *x += 1;
        print!("{x} ");
    }
    println!("\n{:?}", vec);
    println!("\n");

    let mut vec1 = vec![1, 2, 3, 4];
    let vec2 = vec![10, 20, 30];
    vec1.extend(vec2.clone());
    println!("vec1: {vec1:?}");
    println!("vec2: {vec2:?}");
    vec1.extend(vec2);  // vec2 is exhausted here
    println!("vec1: {vec1:?}");
    println!("\n");

    // use std::collections::VecDeque;
    let vec = vec![1, 2, 3, 4];
    let buf: VecDeque<_> = vec.into_iter().collect();
    println!("buf: {buf:?}");
    println!("\n");

    let vec = vec![1, 2, 3, 4];
    for x in vec.iter().rev() {
        print!("{x} ");
    }
    println!("\n");

    /// Counting the number of times each character in a string occurs
    // use std::collections::btree_map::BTreeMap;
    let mut count = BTreeMap::new();
    let message = "she sells sea shells by the sea shore";
    for c in message.chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    assert_eq!(count.get(&'s'), Some(&8));
    println!("Number of occurrences of each character:");
    for (char, cnt) in &count {
        println!("{char}: {cnt}");
    }
    println!("\n");

    /// Tracking the inebriation of customers at a bar

    // A client of the bar. They have a blood alcohol level.
    struct Person {
        blood_alcohol: f32,
    }

    // All the orders made to the bar, by client ID.
    let orders = vec![1, 2, 1, 2, 3, 4, 1, 2, 2, 3, 4, 1, 1, 1];

    // Our clients.
    let mut blood_alcohol = BTreeMap::new();

    for id in orders {
        // If this is the first time we've seen this customer, initialize them
        // with no blood alcohol. Otherwise, just retrieve them.
        let person = blood_alcohol.entry(id).or_insert(Person { blood_alcohol: 0.0 });

        // Reduce their blood alcohol level. It takes time to order and drink a beer!
        person.blood_alcohol *= 0.9;

        // Check if they're sober enough to have another beer.
        if person.blood_alcohol > 0.3 {
            // Too drunk... for now.
            println!("Sorry {id}, I have to cut you off.");
        } else {
            // Have another!
            person.blood_alcohol += 0.1;
        }
    }

    println!("\n");
}

fn book_vectors() {
    let mut v: Vec<i32> = Vec::new();
    v.push(4);
    v.push(5);
    v.push(6);
    println!("v: {v:?}");
    let mut v: Vec<i32> = Vec::new();
    v.extend(vec![3, 2, 1].iter().rev());
    println!("v: {v:?}");
    let mut v = Vec::new();
    v.push(4);
    v.extend(vec![3, 2, 1]);
    v.push(0);
    println!("v: {v:?}");
    let third = v[2];
    println!("third = {third}");
    let third = &v[2];
    println!("third = {}", third);
    let third: &i32 = &v[2];
    println!("third = {}", third);
    let fourth = v.get(3);
    println!("fourth = {:?}", fourth);
    let fourth: Option<&i32> = v.get(3);
    match fourth {
        Some(fourth) => println!("fourth = {}", fourth),
        None => println!("There is no fourth element.")
    }
    let oob: Option<&i32> = v.get(99);
    match oob {
        Some(oob) => println!("{oob}"),
        None => println!("There is no 99th element.")
    }
    // We can’t have mutable and immutable references in the same scope.
    v.push(-1);
    // println!("third = {third}");  // cannot borrow `v` as mutable because it is also borrowed as immutable
    let v = vec![4, 6, 8];
    for n_ref in &v {
        print!("{n_ref} ")
    }
    println!();
    for x in v.iter().rev() {
        print!("{x} ")
    }
    println!();
    let mut v = vec![7, 8, 9];
    for n_ref in &mut v {
        // n_ref has type &mut i32
        *n_ref += 1;
    }
    println!("{:?}", v);
    println!("\n");

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
    println!("{:?}", row);
    println!("\n");
}

fn book_hashmap() {
    // use std::collections::HashMap;

    let team_name = String::from("Blue");

    let mut scores = HashMap::new();
    scores.insert(team_name.clone(), 10);
    scores.insert(String::from("Yellow"), 50);

    let score = scores.get(&team_name).unwrap();
    println!("score = {score}");
    let score = scores.get(&team_name);
    println!("score = {score:?}");
    match score {
        Some(val) => println!("{val}"),
        None => println!("None")
    }
    scores.insert(team_name.clone(), 11);
    let score = scores.get(&(team_name.clone() + "xxx"));
    println!("score = {score:?}");
    match score {
        Some(val) => println!("{val}"),
        None => println!("None")
    }
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score = {score:?}");
    let score = scores.get(&(team_name.clone() + "xxx")).copied().unwrap_or(0);
    println!("score = {score:?}");
    println!("\n");

    // This code will print each pair in an arbitrary order:
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    for (key, value) in scores.iter() {
        println!("{key}: {value}");
    }
    println!("scores: {:#?}", scores);
    println!("\n");

    let mut map = HashMap::new();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    map.insert(&field_name, &field_value);
    println!("{field_name}: {field_value}");
    println!("\n");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Blue"));
    scores.entry(String::from("Yellow"));
    println!("scores: {:#?}", scores);
    scores.entry(String::from("Blue")).or_insert(11);
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("scores: {:#?}", scores);
    println!("\n");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map = {map:#?}");
    println!("\n");

    let mut h = HashMap::new();
    h.insert("k1", 0);
    let v1 = h["k1"];
    h.insert("k2", 1);
    let v2 = &h["k2"];
    println!("{} {}", v1, v2);
    println!("\n");
}

/// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
/// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
fn median_mode(array: &[i32]) {
    let mut vec = Vec::from(array);
    println!("{:?}", array);
    println!("{:?}", vec);

    vec.sort();
    println!("{:?}", vec);
    let length = vec.len();
    println!("length = {length}");
    let median_index = length / 2;
    let median = vec[median_index];
    println!("median = {median}; median_index = {median_index}");

    let mut counter = HashMap::new();
    for num in vec.iter() {
        let count = counter.entry(*num).or_insert(0);
        *count += 1;
    }
    println!("counter: {:?}", counter);

    println!("\n");
}

/// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added,
/// so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead
/// (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
fn pig_latin(word: &str) -> String {
    println!("original word = {word}");
    let mut word = word.trim().split_whitespace().next().unwrap().to_string().to_lowercase();
    println!("trimmed string lower-cased word = {word}");

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    // let vowels: Vec<char> = "aeiou".chars().collect();
    // println!("{vowels:?}");

    // pattern in `starts_with` doesn't support vectors, but it supports arrays
    if word.starts_with(vowels) {
        word + "-hay"
    } else {
        let first = word.chars().next().unwrap().to_string();
        word = word[1..word.len()].to_string();
        word += "-";
        word += &first;
        word += "ay";
        word
    }
}

fn main() {
    documentation_examples();
    book_vectors();
    book_hashmap();

    let array = [5, 7, 3, 5, 6, 4, 4, 2, 0, 5, 1, 8, 7, 2, -1];
    median_mode(&array);

    let word = String::from("  first  \n ");
    let word = "  First Second ";
    let word = pig_latin(&word);
    println!("pig-latin converted word = {word}");
    println!();
    let word = "ApPle";
    let word = pig_latin(&word);
    println!("pig-latin converted word = {word}");
    println!();
    println!("\n");
}

#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

const FILE_NAME: &str = "hello.txt";


/// Uses `match` to handle opening a file.
fn ver1() {
    let file_name = FILE_NAME;
    let greeting_file_result = File::open(file_name);
    println!("greeting_file_result: {greeting_file_result:?}");
    let greeting_file = match greeting_file_result {
        Ok(handle) => handle,
        Err(error) => panic!("Problem opening the file {:?}; Error: {:?}", file_name, error),
    };
    println!("greeting_file: {greeting_file:?}");
}

/// Uses nested `match`-es to handle opening or creating a file.
fn ver2() {
    let file_name = FILE_NAME;
    let greeting_file_result = File::open(file_name);

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => fc,
                Err(err) => panic!("Problem creating the file {:?}; Error: {:?}", file_name, err),
            },
            other_error=> panic!("Problem opening the file {:?}; Error: {:?}", file_name, other_error),
        }
    };
}

/// Uses `unwrap_or_else`, and inside of it, `if-else`.
fn ver3a() {
    let file_name = FILE_NAME;

    let greeting_file = File::open(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|err| {
                panic!("Problem creating the file {:?}; Error: {:?}", file_name, err);
            })
        } else {
            panic!("Problem opening the file {:?}; Error: {:?}", file_name, error);
        }
    });
}

/// Uses `unwrap_or_else`, and inside of it, a `match`.
fn ver3b() {
    let file_name = FILE_NAME;

    let greeting_file = File::open(file_name).unwrap_or_else(|error| {
    match error.kind() {
            ErrorKind::NotFound => File::create(file_name).unwrap_or_else(|err| {
                panic!("Problem creating the file {:?}; Error: {:?}", file_name, err);
            }),
        other_error=> panic!("Problem opening the file {:?}; Error: {:?}", file_name, other_error),
        }
    });
}

/// Panics if the file doesn't exist.
fn ver4() {
    /// `unwrap()` outputs a generic message, which we cannot customize (change).
    // let greeting_file = File::open(FILE_NAME).unwrap();

    let msg = String::from("\"") + FILE_NAME + "\" should be included in this project";
    let msg = msg.as_str();

    /// Just use the `format!` macro, instead.
    let msg = format!("\"{FILE_NAME}\" should be included in this project");

    /// In contrast to `unwrap()`, `expect()` takes and outputs our custom message, which can contain more detail.
    /// We should describe the reason we expect the Result should be Ok.
    let greeting_file = File::open(FILE_NAME).expect(&msg);
}

/// A Function that reads a username from a file.
/// If the file doesn't exist or can't be read,
/// this function will return those errors to the code that called the function.
///
/// A function that returns errors to the calling code using match
fn read_username_from_file_v1() -> Result<String, io::Error> {
    let username_file_result = File::open(FILE_NAME);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/// A function that returns errors to the calling code using the ? operator
fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username_file = File::open(FILE_NAME)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

/// Chaining method calls after the ? operator
fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(FILE_NAME)?.read_to_string(&mut username)?;
    Ok(username)
}

/// Using `fs::read_to_string` instead of opening and then reading the file
fn read_username_from_file_v4() -> Result<String, io::Error> {
    fs::read_to_string(FILE_NAME)
}

/// Wrapper around `read_username_from_file_v*()` variants.
/// Handles the result returned from `read_username_from_file_v*()`, whether it is an `Ok` or an `Err`.
fn handle_propagation() {
    let result = read_username_from_file_v4();
    println!("{:?}", result);
}

/// Using the ? operator on an Option<T> value
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn test_last_char_of_first_line() {
    assert_eq!(
        last_char_of_first_line("Hello, world\nHow are you today?"),
        Some('d')
    );

    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);
}

/// Guessing game
fn guessing_game_v1() {
    println!("\nGuess the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            }
        }
    }
}

/// A Guess type that will only continue with values between 1 and 100
pub struct Guess {
    value: i32,  // private field
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    /// Getter
    pub fn value(&self) -> i32 {
        self.value
    }
}

/// Guessing game
fn guessing_game_v2() {
    println!("\nGuess the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess);

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // ver3b();
    // ver4();

    handle_propagation();

    test_last_char_of_first_line();

    // guessing_game_v2();

    /// We have to change the `main`'s return value to `Result<(), Box<dyn Error>>`
    /// because of the `?` after `File::open()`, and we also have to return `Ok(())`.
    /// For now, you can read Box<dyn Error> to mean “any kind of error.”
    /// This is just a demo to show that `main()` can return a `Result<(), E>`.
    let greeting_file = File::open(FILE_NAME)?;
    Ok(())
}

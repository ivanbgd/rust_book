fn increment(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn simple_match() {
    println!();
    let mut x = Some(-8);
    let mut y = increment(x);
    println!("y = {}", y.unwrap());

    x = None;
    y = increment(x);
    println!("y = {y:?}");
}

/// Mixing `if let`, `else if`, `else if let` and `else`.
fn conditional_if_let() {
    println!();
    let favorite_color: Option<&str> = None;  // Some("red");
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();  // "-34" or "aaa" don't break program, but print "blue" instead!

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background.");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using orange as the background color.");
        } else {
            println!("Using purple as the background color.");
        }
    } else {
        println!("Using blue as the background color.");
    }
}

/// Using a `while let` loop to print values for as long as `stack.pop()` returns `Some`.
fn conditional_while_let_loop() {
    println!();
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

/// Using a pattern in a for loop to destructure a tuple.
fn for_loop() {
    println!();
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

/// Using a pattern to destructure a tuple and create three variables at once.
fn let_statements() {
    println!();

    let (x, y, z) = (4, 'e', "www");
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);

    let (a, .., b) = (-14, 'e', "www", 67.32);
    println!("{}", a);
    println!("{}", b);
}

/// A function with parameters that destructure a tuple.
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!();
    println!("Coordinates are: ({}, {})", x, y);
    println!("Updated coordinates are: {:?}", (|&(a, b)| {
        (a + 1, b + 1)
    })(&(x, y)));
    println!("Updated coordinates are: {:?}", (|| {
        (x + 2, y + 2)
    })());
    println!("Coordinates are: ({}, {})", x, y);
}

fn quiz1() {
    println!();

    let mut v = vec![(1, 2), (3, 4)].into_iter();
    let mut sum = 0;
    while let Some(tuple) = v.next() {
        let (_, n) = tuple;
        sum += n;
    }
    println!("sum = {sum}");

    let mut v = vec![(10, 20), (30, 40)].into_iter();
    let mut sum = 0;
    while let Some((_, n)) = v.next() {
        sum += n;
    }
    println!("sum = {sum}");
}

fn refutability() {
    println!();
    let some_option_value: Option<i32> = None;

    // Using if let and a block with refutable patterns instead of let
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    // Attempting to use an irrefutable pattern with if let
    if let x = 5 {
        println!("{}", x);
    }
}

fn matching() {
    println!();
    println!();

    let x = 2;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    let x = 2;
    match x {
        1 | 2 => println!("one OR two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'g';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn destructuring() {
    println!();

    struct Point {
        x: i32,
        y: i32,
    }

    const X: i32  = 0;
    const Y: i32  = 5;

    let p = Point { x: X, y: Y };

    // Destructuring a struct’s fields into separate variables
    let Point { x: a, y: b } = p;
    assert_eq!(X, a);
    assert_eq!(Y, b);
    let Point { x: x, y: y } = p;
    assert_eq!(X, x);
    assert_eq!(Y, y);
    // Destructuring struct fields using struct field shorthand
    let Point {x, y } = p;
    assert_eq!(X, x);
    assert_eq!(Y, y);

    // Destructuring and matching literal values in one pattern
    match p {
        Point { x: 0, y } => println!("On the y-axis at {y}"),
        Point { x, y: 0 } => println!("On the x-axis at {x}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    // Destructuring enum variants that hold different kinds of values
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => println!("Move in the x direction {x} and in the y direction {y}."),
        Message::Write(text) => println!("Text message: {text}."),
        Message::ChangeColor(r, g, b) => println!("Change the color to red {r}, green {g}, and blue {b}."),
    }

    enum Color {
        Rgb(u8, u8, u8),
        Hsv(u8, u8, u8),
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message2::ChangeColor(Color::Hsv(10, 20, 30));

    // Matching on nested enums
    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) =>
            println!("Change color to red {r}, green {g}, and blue {b}."),
        Message2::ChangeColor(Color::Hsv(h, s, v)) =>
            println!("Change color to hue {h}, saturation {s}, value {v}."),
        _ => (),
    }

    let ((feet, inches), Point { x, y }) = ((30, 45), Point { x: 17, y: -27 });
    println!("feet: {}, inches: {}; ({}, {})", feet, inches, x, y);
}

fn ignoring_values() {
    println!();

    fn foo(_:i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value."),
        _ => setting_value = new_setting_value,
    }
    println!("Setting is: {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => println!("{first}, {third}, {fifth}"),
    }

    // Starting a variable name with an underscore to avoid getting unused variable warnings
    let _unused = 10;

    let s = Some(String::from("Hello!"));
    // We need a reference to s.
    if let Some(_s) = &s {
        println!("Found a string: {_s}");
    }
    println!("{:?}", s);

    let s = Some(String::from("Hello!"));
    // We don't need a reference to s.
    if let Some(_) = s {
        println!("Found a string: {s:?}");
    }
    println!("{:?}", s);

    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    match numbers {
        (first, .., last) => println!("{first}, {last}"),
    }
}

fn main() {
    simple_match();
    conditional_if_let();
    conditional_while_let_loop();
    for_loop();
    let_statements();
    let point = (30, 50);
    print_coordinates(&point);
    quiz1();

    refutability();

    matching();
    destructuring();
    ignoring_values();
}

use std::fmt::{self, Debug, Display, Formatter};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("\"{}\", by @{}", self.content, self.username)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{} wrote:\n\"{}\"\nreply: {}; retweet: {}",
               self.username, self.content, self.reply, self.retweet)
    }
}

impl<T: fmt::Debug> Summary for Vec<T> {
    fn summarize_author(&self) -> String {
        unimplemented!()
    }

    fn summarize(&self) -> String {
        format!("The vector's length is {}. Contents are: {:?}", self.len(), self)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/// item1 and item2 can have different types, but both need to implement `Summary`.
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news!\n{}\n{}", item1.summarize(), item2.summarize());
}

/// item1 and item2 must be the same type (due to the trait bound), and the type has to implement `Summary`.
pub fn notify4<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news!\n{}\n{}", item1.summarize(), item2.summarize());
}

/// The type `T` needs to implement the `Summary` trait, and the `Display` trait,
/// whether it comes from the standard library, or it's our own implementation of the `Display` trait.
/// In our case, only `Tweet` implements `Display`; `NewsArticle` or `Vec<T>` don't,
/// so we can't use this function on them.
pub fn notify5(item: &(impl Summary + Display)) {
    println!("Breaking news!\n{}", item.summarize());
}

/// Alternative function signature to `notify5`, but here we also actually use the `Display` trait.
/// The type `T` needs to implement the `Summary` trait, and the `Display` trait,
/// whether it comes from the standard library, or it's our own implementation of the `Display` trait.
/// In our case, only `Tweet` implements `Display`; `NewsArticle` or `Vec<T>` don't,
/// so we can't use this function on them.
pub fn notify6<T: Summary + Display>(item: &T) {
    println!("Breaking news!\n{}", item);
}

pub fn some_function1<T: Display + Clone + std::ops::Add<U, Output=i32>, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    t.clone() + u.clone()
}

/// Alternative function signature to `some_function1`, which is easier to read.
pub fn some_function2<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone + std::ops::Add<U, Output=i32>,
        U: Clone + Debug
{
    t.clone() + u.clone()
}

/// Returns a `Tweet`.
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("my_username2"),
        content: String::from(
            "Demo content from `returns_summarizable`",
        ),
        reply: true,
        retweet: false,
    }
}

// #[derive(Debug)]
pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T: Display> Display for Pair<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\tDisplay Pair {{\tx: {}, y: {}\t}}", self.x, self.y)
    }
}

impl<T: Display> Debug for Pair<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\tDebug Pair {{\tx: {}, y: {}\t}}", self.x, self.y)
    }
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/// Using Trait Bounds to Conditionally Implement Methods
/// Conditionally implementing methods on a generic type depending on trait bounds
impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_and_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}.", self.x);
        } else {
            println!("The largest member is y = {}.", self.y);
        }
    }
}

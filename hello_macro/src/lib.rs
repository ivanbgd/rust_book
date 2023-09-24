pub trait HelloMacro {
    fn hello_macro();
}

#[cfg(test)]
mod tests {
    use super::*;
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct PancakesTest;

    #[test]
    fn test_hello_macro() {
        println!();
        println!("test_hello_macro:");
        PancakesTest::hello_macro();  // Hello, Macro! My name is Pancakes!
    }
}

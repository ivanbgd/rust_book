pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Box<dyn Draw> is a trait object; it's a stand-in for any type inside a Box that implements the Draw trait.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("Draw Button");
    }
}

impl Button {
    pub fn on_click(&self) {
        // code to handle the click event
        println!("Click Button");
    }
}

/// `Draw` for `String` needs to be implemented in this crate.
impl Draw for String {
    fn draw(&self) {
        println!("Draw String: \"{}\"", self);
    }
}

mod screen_generic {
    #![allow(unused)]

    use crate::Draw;

    struct ScreenGeneric<T: Draw> {
        components: Vec<T>,
    }

    impl<T> ScreenGeneric<T>
        where T: Draw
    {
        fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
}

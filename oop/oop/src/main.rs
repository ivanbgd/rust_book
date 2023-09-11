use crate::avg_coll_mod::averaged_collection_example;
use crate::blog_oop_mod::blog_oop_example;
use crate::blog_rust_mod::blog_rust_example;
use crate::gui_mod::{gui_example, gui_example2};

mod avg_coll_mod {
    pub fn averaged_collection_example() {
        println!();
        let mut ac = averaged_collection::AveragedCollection::new();
        println!("{}\n", ac);
        ac.remove();
        println!("{:#?}\n", ac);
        ac.add(8);
        ac.add(-3);
        ac.add(6);
        ac.add(-7);
        ac.add(0);
        ac.add(12);
        println!("{:?}\n", ac);
        ac.sort();
        println!("{:#?}\n", ac);
        ac.remove();
        println!("{}\n", ac);
        println!("{}", ac.average());
    }
}

mod gui_mod {
    #![allow(unused)]

    use gui::{Button, Draw, Screen};

    struct SelectBox {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            // code to actually draw a select box
            println!("Draw SelectBox");
        }
    }

    pub fn gui_example() {
        println!();

        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    x: 10,
                    y: 10,
                    width: 45,
                    height: 45,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    x: 100,
                    y: 100,
                    width: 25,
                    height: 15,
                    label: String::from("Ok"),
                }),
                Box::new(
                    String::from("Hi! We have implemented the `Draw` trait for `String`!")
                ),
            ],
        };

        screen.run();

        let button = &screen.components[1];
        // no method named `on_click` found for trait object `dyn Draw` in the current scope
        // (**button).on_click();

        // Unlike some OOP languages, a trait object cannot be "downcast"
        // to a more concrete type (except in the case of the Any trait).
        // non-primitive cast: `Box<(dyn Draw + 'static)>` as `Button`
        // an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
        // let button = screen.components[1] as Button;
    }

    pub fn gui_example2() {
        println!();

        let components: Vec<Box<dyn Draw>> = vec![
            Box::new(SelectBox {
                x: 10,
                y: 10,
                width: 45,
                height: 45,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                x: 100,
                y: 100,
                width: 25,
                height: 15,
                label: String::from("Ok"),
            }),
            Box::new(
                String::from("Hi! We have implemented the `Draw` trait for `String`!")
            ),
        ];

        let screen = Screen {
            components
        };

        screen.run();
    }
}

mod blog_oop_mod {
    use blog_oop::Post;

    const CONTENT: &str = "This is an example OOP blog post content.";
    const IMPROVED_CONTENT: &str = "This is an example OOP blog post content. IMPROVED!";

    pub fn blog_oop_example() {
        println!();
        let mut post = Post::new();

        post.add_text(CONTENT);
        assert_eq!("", post.content());

        // No effect.
        post.reject();
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        // Reject, moving it back to "Draft".
        post.reject();
        assert_eq!("", post.content());

        post.add_text(" IMPROVED!");
        assert_eq!("", post.content());

        // We have to call `request_review()` after each rejection, because it's a draft again.
        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("", post.content());

        // Reject, moving it back to "Draft".
        post.reject();
        assert_eq!("", post.content());

        // Has no effect on a draft.
        post.approve();
        assert_eq!("", post.content());

        // We have to call `request_review()` after each rejection, because it's a draft again.
        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_ne!(IMPROVED_CONTENT, post.content());
        assert_eq!("", post.content());

        post.approve();
        assert_eq!(IMPROVED_CONTENT, post.content());

        println!("{}", IMPROVED_CONTENT);
    }
}

mod blog_rust_mod {
    use blog_rust::Post;

    const CONTENT: &str = "This is an example Rust blog post content.";
    const IMPROVED_CONTENT: &str = "This is an example Rust blog post content. IMPROVED!";

    pub fn blog_rust_example() {
        println!();
        let mut post = Post::new();

        post.add_text(CONTENT);

        let post = post.request_review();

        // Reject, moving it back to "Draft".
        let mut post = post.reject();

        post.add_text(" IMPROVED!");

        // We have to call `request_review()` after each rejection, because it's a draft again.
        let post = post.request_review();

        let post = post.approve();

        // Reject, moving it back to "Draft".
        let mut post = post.reject();
        post.add_text("");

        // We have to call `request_review()` after each rejection, because it's a draft again.
        let post = post.request_review();

        let post = post.approve();
        let post = post.approve();
        assert_ne!(CONTENT, post.content());
        assert_eq!(IMPROVED_CONTENT, post.content());

        println!("{}", IMPROVED_CONTENT);
    }
}

fn main() {
    averaged_collection_example();
    gui_example();
    gui_example2();
    blog_oop_example();
    blog_rust_example();
}

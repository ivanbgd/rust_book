use std::fmt::{Debug, Display, Formatter};

enum List1 {
    Cons1(i32, Box<List1>),
    Nil1,
}

use crate::List1::{Cons1, Nil1};

struct MyBox<T> (T);  // The MyBox type is a tuple struct with one element of type T.

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

/// We are "overloading", in C++ terms, the dereferencing operator, `*`.
impl<T> Deref for MyBox<T> {
    type Target = T;  // an associated type; this is mandatory

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

/// A destructor, in C++ terms, or perhaps a finalizer.
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

use std::rc::Rc;

#[derive(Debug)]
enum List2 {
    Cons2(i32, Rc<List2>),
    Nil2
}

impl List2 {
    fn head_value(&self) -> Option<i32> {
        match self {
            Cons2(value, _) => Some(*value),
            Nil2 => None,
        }
    }

    fn modify_head_value(&mut self, new_value: i32) {
        match self {
            Cons2(value, _) => *value = new_value,
            Nil2 => ()
        }
    }

    fn head_list(&self) -> Option<&Rc<List2>> {
        match self {
            Cons2(_, list) => Some(list),
            Nil2 => None,
        }
    }
}

use crate::List2::{Cons2, Nil2};

use std::cell::{RefCell, RefMut};

#[derive(Debug)]
enum List3 {
    Cons3(Rc<RefCell<i32>>, Rc<List3>),
    Nil3,
}

impl List3 {
    fn head_value(&self) -> Option<i32> {
        match self {
            Cons3(value, _) => Some(*value.borrow()),
            Nil3 => None,
        }
    }

    fn modify_head_value(&mut self, new_value: i32) {
        match self {
            Cons3(value, _) => *value.borrow_mut() = new_value,
            Nil3 => (),
        }
    }

    fn head_list(&self) -> Option<&Rc<List3>> {
        match self {
            Cons3(_, list) => Some(list),
            Nil3 => None,
        }
    }
}

use crate::List3::{Cons3, Nil3};

/// A cons list definition that holds a RefCell<T> so we can modify what a Cons variant is referring to.
#[derive(Debug)]
enum List4 {
    Cons4(i32, RefCell<Rc<List4>>),
    Nil4,
}

impl List4 {
    fn tail(&self) -> Option<&RefCell<Rc<List4>>> {
        match self {
            Cons4(_, item) => Some(item),
            Nil4 => None,
        }
    }
}

use crate::List4::{Cons4, Nil4};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    println!();
    let _list = Cons1(1, Box::new(Cons1(2, Box::new(Cons1(3, Box::new(Nil1))))));

    println!();
    let x = 5;
    let y = &x;
    assert_eq!(*y, x);
    let y = Box::new(5);
    assert_eq!(*y, x);
    let y = MyBox::new(5);
    assert_eq!(*y, x);
    assert_eq!(*y.deref(), x);
    assert_eq!(*(y.deref()), x);

    println!();
    hello("Pera");
    hello(&String::from("Mika"));
    hello(&Box::new("Zika"));
    hello(&Box::new(String::from("Laza")));
    hello(&MyBox::new("Ivan"));
    hello(&MyBox::new(String::from("Nenad")));
    let m = MyBox::new(String::from("Rust"));
    hello(&*m);
    hello(&(*m));
    hello(&(*m)[..]);

    println!();
    let c = CustomSmartPointer {
        data: String::from("c: my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("d: other stuff"),
    };
    println!("CustomSmartPointers created.");  // CustomSmartPointers created.
    println!("CustomSmartPointers created: '{}', '{}'", c.data, d.data);  // CustomSmartPointers created: 'c: my stuff', 'd: other stuff'
    // drop(c);
    println!("CustomSmartPointers created: '{}'", d.data);  // CustomSmartPointers created: 'd: other stuff'
    // `d` is deallocated first, before `c`, if we don't first explicitly drop `c`.

    println!();
    let mut value = 5;
    // We are not holding a reference to `value`.
    let /*mut*/ a = Rc::new(Cons2(value, Rc::new(Cons2(10, Rc::new(Nil2)))));
    let b = Cons2(3, Rc::clone(&a));
    let c = Cons2(4, Rc::clone(&a));
    // let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    // let b = Rc::new(Cons2(3, Rc::clone(&a)));
    // let c = Rc::new(Cons2(4, Rc::clone(&a)));
    value = 15;
    println!("00 a = {:?}", a);  // value = 5; a = Cons2(5, Cons2(10, Nil2))
    println!("00 b = {:?}", b);  // value = 5; b = Cons2(3, Cons2(5, Cons2(10, Nil2)))
    println!("00 c = {:?}", c);  // value = 5; c = Cons2(4, Cons2(5, Cons2(10, Nil2)))
    println!("01 Nil2 = {:?}", List2::Nil2);  // Nil2 = Nil2
    println!("01 Cons2 = {:?}", List2::Cons2(66, Rc::new(Nil2)));  // Cons2 = Cons2(66, Nil2)
    println!("02 a.value = {:?}", a.head_value());  // a.value = Some(5)
    println!("02 b.value = {:?}", b.head_value());  // b.value = Some(3)
    println!("02 c.value = {}", c.head_value().expect("Expected c.value."));  // c.value = 4
    println!("03 a.list = {:?}", a.head_list());  // a.list = Some(Cons2(10, Nil2))
    println!("03 b.list = {:?}", b.head_list());  // b.list = Some(Cons2(5, Cons2(10, Nil2)))
    println!("03 c.list = {:?}", c.head_list().expect("Expected c.list."));  // c.list = Cons2(5, Cons2(10, Nil2)) - Same as b.list, as expected.
    // a.modify_head_value(111);
    println!("04 a.value = {:?}", a.head_value());  // a.value = Some(5)

    println!();
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("Count after creating a = {}.", Rc::strong_count(&a));  // 1
    let b = Cons2(3, Rc::clone(&a));
    println!("Count after creating b = {}.", Rc::strong_count(&a));  // 2
    {
        let c = Cons2(3, Rc::clone(&a));
        println!("Count after creating c = {}.", Rc::strong_count(&a));  // 3
    }
    println!("Count after c goes out of scope = {}.", Rc::strong_count(&a));  // 2

    println!();
    let mut x = 5;
    let y = &mut x;

    println!();
    let value = Rc::new(RefCell::new(5));
    // In a, we need to clone value so both a and value have ownership of the inner 5 value
    // rather than transferring ownership from value to a or having a borrow from value.
    // We wrap the list a in an Rc<T> so when we create lists b and c, they can both refer to a.
    let a = Rc::new(Cons3(Rc::clone(&value), Rc::new(Nil3)));
    let mut b = Cons3(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons3(Rc::new(RefCell::new(4)), Rc::clone(&a));
    *(value.borrow_mut()) = 16;  // value.borrow_mut() dereferences value, which is an Rc<T>, to the inner RefCell<T>.
    *value.borrow_mut() = 17;
    println!("000 a = {:?}", a);  // a = Cons3(RefCell { value: 17 }, Nil3)
    println!("000 b = {:?}", b);  // b = Cons3(RefCell { value: 3 }, Cons3(RefCell { value: 17 }, Nil3))
    println!("000 c = {:?}", c);  // c = Cons3(RefCell { value: 4 }, Cons3(RefCell { value: 17 }, Nil3))
    println!("002 a.value = {:?}", a.head_value());  // a.value = Some(17)
    println!("002 b.value = {:?}", b.head_value());  // b.value = Some(3)
    println!("002 c.value = {}", c.head_value().expect("Expected c.value."));  // c.value = 4
    println!("003 a.list = {:?}", a.head_list());  // a.list = Some(Nil3)
    println!("003 b.list = {:?}", b.head_list());  // b.list = Some(Cons3(RefCell { value: 17 }, Nil3))
    println!("003 c.list = {:?}", c.head_list().expect("Expected c.list."));  // c.list = Cons3(RefCell { value: 17 }, Nil3) - Same as b.list, as expected.
    // Rc::get_mut(&mut a).unwrap().modify_head_value(111);
    if let Some(mut value) = a.head_value() {
        value = 44;  // This compiles and runs, but doesn't modify value in a. This value is a local variable.
    }
    b.modify_head_value(222);
    println!("004 a.value = {}", a.head_value().expect("Expected a.value."));  // a.value = 17
    println!("004 b.value = {}", b.head_value().expect("Expected b.value."));  // b.value = 222

    println!();
    let a = Rc::new(Cons4(5, RefCell::new(Rc::new(Nil4))));
    println!("0000 a = {:?}", a);  // Cons4(5, RefCell { value: Nil4 })
    println!("a initial RC count = {}", Rc::strong_count(&a));  // a initial RC count = 1
    println!("a next item = {:?}", a.tail());  // a next item = Some(RefCell { value: Nil4 })
    // let b = Cons4(3, RefCell::new(Rc::clone(&a))); // This increases RC count in a, even though we redefine b later! But, we can't count references with this.
    // println!("0000 b = {:?}", b);  // Cons4(3, RefCell { value: Cons4(5, RefCell { value: Nil4 }) })
    let b = Rc::new(Cons4(3, RefCell::new(Rc::clone(&a))));  // Initial RC count in b is 1 regardless of the above definition of b. We need this variant, though, so we can count references to it!
    println!("0000 b = {:?}", b);  // Cons4(3, RefCell { value: Cons4(5, RefCell { value: Nil4 }) }) - same as above
    println!("a RC count after creating b = {}", Rc::strong_count(&a));  // a RC count after creating b = 2 (or 3!)
    println!("b initial RC count = {}", Rc::strong_count(&b));  // b initial RC count = 1
    println!("b next item = {:?}", b.tail());  // b next item = Some(RefCell { value: Cons4(5, RefCell { value: Nil4 }) })
    let link = a.tail().unwrap();
    println!("link = {:?}", link);
    if let Some(link) = a.tail() {
        // This creates a reference cycle of lists a and b pointing to each other.
        println!("link = {:?}", link);
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b RC count after changing a = {}", Rc::strong_count(&b));  // b RC count after changing a = 2
    println!("a RC count after changing a = {}", Rc::strong_count(&a));  // a RC count after changing a = 2
    // println!("0001 a = {:?}", a);  // a = Cons4(5, RefCell { value: Cons4(3, RefCell { value: Cons4(5, RefCell { value: Cons4(3, RefCell {... - stack overflow!
    // println!("a next item = {:?}", a.tail());  // a next item = Some(RefCell { value: Cons4(3, RefCell { value: Cons4(5, RefCell { value: Cons4(3, RefCell { value: Cons4(5, RefCell {... - stack overflow!
    // println!("0001 b = {:?}", b);  // b = Cons4(3, RefCell { value: Cons4(5, RefCell { value: Cons4(3, RefCell { value: Cons4(5, RefCell {... - stack overflow!
    // println!("b next item = {:?}", b.tail());  // b next item = Some(RefCell { value: Cons4(5, RefCell { value: Cons4(3, RefCell { value: Cons4(5, RefCell { value: Cons4(3, RefCell {... - stack overflow!

    tree_no_parent();
    tree();
    tree2();

    println!();
    println!();

    /*
    The reference count of the Rc<List> instances in both a and b are 2 after we change the list in a to point to b.
    At the end of main, Rust drops the variable b, which decreases the reference count of the b Rc<List> instance from 2 to 1.
    The memory that Rc<List> has on the heap won’t be dropped at this point, because its reference count is 1, not 0.
    Then Rust drops a, which decreases the reference count of the a Rc<List> instance from 2 to 1 as well.
    This instance’s memory can’t be dropped either, because the other Rc<List> instance still refers to it.
    The memory allocated to the list will remain uncollected forever.
    */

    // Dropping CustomSmartPointer with data 'd: other stuff'!
    // Dropping CustomSmartPointer with data 'c: my stuff'!
}

/// We can modify `value` in `Node1`, through `RefCell`.
#[derive(Debug)]
struct Node1 {
    value: RefCell<i32>,
    children: RefCell<Vec<Rc<Node1>>>,
}

fn tree_no_parent() {
    println!();
    println!("*********************************************\n");

    let leaf = Rc::new(Node1 {
        value: RefCell::new(3),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node1 {
        value: RefCell::new(5),
        children: RefCell::new(vec![]),
    });

    *branch.value.borrow_mut() = 10;
    branch.children.borrow_mut().push(Rc::clone(&leaf));
    println!("branch = {:?}", branch);
    println!("leaf = {:?}", leaf);

    println!();
}

use std::rc::Weak;

/// We can't modify `value` in `Node`.
// #[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
   children: RefCell<Vec<Rc<Node>>>,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let parent = match self.parent.borrow().upgrade() {
            // Some(p) => format!("{}", p),
            Some(p) => format!("p.v: {}", p.value),
            None => String::from("N/A"),
        };
        write!(f, "% v: {}, p: {}, c: {:#?} %", self.value, parent, self.children.borrow())
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let parent = match self.parent.borrow().upgrade() {
            // Some(p) => format!("{}", p),
            Some(p) => format!("p.v: {}", p.value),
            None => String::from("N/A"),
        };
        write!(f, "% v: {}, p: {}, c: {:?} %", self.value, parent, self.children.borrow())
    }
}

fn tree() {
    println!();
    println!("*********************************************\n");

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf's strong and weak counts: {}, {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));  // 1, 0
    println!("leaf's parent = {:?}", leaf.parent.borrow().upgrade());  // leaf's parent = None  // We can't unwrap() None.
    println!("leaf's strong and weak counts: {}, {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));  // 1, 0
    println!();

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    println!("branch's strong and weak counts 0:\t{}, {}", Rc::strong_count(&branch), Rc::weak_count(&branch));  // 1, 0

    // This increases the branch's weak count by 1.
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("branch's strong and weak counts 1:\t{}, {}", Rc::strong_count(&branch), Rc::weak_count(&branch));  // 1, 1

    println!("leaf's strong and weak counts: {}, {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));  // 2, 0
    // We can unwrap here, if we like.
    println!("leaf's parent = {:?}", leaf.parent.borrow().upgrade());  // leaf's parent = Some(% v: 5, p: N/A, c: [% v: 3, p: p.v: 5, c: [] %] %) - or Some(Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } })
    println!("leaf's strong and weak counts: {}, {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));  // 2, 0
    println!("branch's strong and weak counts 2:\t{}, {}", Rc::strong_count(&branch), Rc::weak_count(&branch));  // 1, 1
    println!();

    println!("branch = {:?}", branch);  // branch = 5, None -> [3, None -> []]  // branch = Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } }
    println!("branch = {}", branch);  // branch = 5, None -> [3, None -> []]  // % v: 5, p: N/A, c: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] %
    println!("leaf = {:?}", leaf);  // leaf = 3, None -> []  // leaf = Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }
    println!("leaf = {}", leaf);  // leaf = 3, None -> []  // leaf = % v: 3, p: p.v: 5, c: [] %

    println!();
    println!("leaf's parent = {:?}", leaf.parent.borrow().upgrade());  // leaf's parent = Some(% v: 5, p: N/A, c: [% v: 3, p: p.v: 5, c: [] %] %)
    println!("leaf's strong and weak counts: {}, {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));  // 2, 0
    println!("branch's strong and weak counts 3:\t{}, {}", Rc::strong_count(&branch), Rc::weak_count(&branch));  // 1, 1

    println!();
}

/*
branch = % v: 5, p: N/A, c: [% v: 3, p: p.v: 5, c: [] %] %
branch = % v: 5, p: N/A, c: [
    % v: 3, p: p.v: 5, c: [] %,
] %
leaf = % v: 3, p: p.v: 5, c: [] %
leaf = % v: 3, p: % v: 5, p: N/A, c: [
    % v: 3, p: p.v: 5, c: [] %,
] %, c: [] %
*/

/*
branch = % v: 5, p: N/A, c: [% v: 3, p: p.v: 5, c: [] %] %
branch = % v: 5, p: N/A, c: [
    % v: 3, p: p.v: 5, c: [] %,
] %
leaf = % v: 3, p: p.v: 5, c: [] %
leaf = % v: 3, p: p.v: 5, c: [] %
*/

// TODO: Implement Display for Vec<Node>. But, it doesn't let me!

// The two below implementations of Display and Debug overflow the stack.
// That doesn't happen with with std::fmt::Debug.
// But, my Debug is prettier, and so is my Display. Unfortunately, they fail if I add the leaf's parent, which is branch.
// This means that we are not breaking the reference cycle between branch and leaf.
// I suspect that this cycle exists due to the infinite printing between a parent and a child.
// We fill the print (stdout) buffer with parent to child to parent to child, etc. connections.
// In the above implementation, I am breaking the cycle by printing only the parent's value.
// We should implement both Display and Debug traits, as I have, to get our fully custom output.
// We can derive the std::fmt::Debug for Node, but then output won't be fully custom, and certainly not the nicest.

// impl Display for Node {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(f, "{}, {:?} -> {:?}", self.value, self.parent.borrow().upgrade(), self.children.borrow())
//     }
// }

// impl Debug for Node {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(f, "{}, {:?} -> {:?}", self.value, self.parent.borrow().upgrade(), self.children.borrow())
//     }
// }

fn tree2() {
    println!();
    println!("*********************************************\n");

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

   println!("leaf's strong and weak counts: {}, {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));  // 1, 0
    println!("leaf's parent = {:?}", leaf.parent.borrow().upgrade());  // leaf's parent = None  // We can't unwrap() None.
    println!();

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        println!("branch's strong and weak counts 0:\t{}, {}", Rc::strong_count(&branch), Rc::weak_count(&branch));  // 1, 0

        // This increases the branch's weak count by 1.
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch's strong and weak counts 1:\t{}, {}", Rc::strong_count(&branch), Rc::weak_count(&branch));  // 1, 1

        println!("leaf's strong and weak counts: {}, {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));  // 2, 0
        // We can unwrap here, if we like.
        println!("leaf's parent = {:?}", leaf.parent.borrow().upgrade());  // leaf's parent = Some(% v: 5, p: N/A, c: [% v: 3, p: p.v: 5, c: [] %] %) - or Some(Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } })
        println!();

        println!("branch = {:?}", branch);  // branch = 5, None -> [3, None -> []]  // branch = Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } }
        println!("branch = {}", branch);  // branch = 5, None -> [3, None -> []]  // % v: 5, p: N/A, c: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] %
        println!("leaf = {:?}", leaf);  // leaf = 3, None -> []  // leaf = Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }
        println!("leaf = {}", leaf);  // leaf = 3, None -> []  // leaf = % v: 3, p: p.v: 5, c: [] %
    }

    // When the inner scope ends, branch goes out of scope and the strong count of the Rc<Node> decreases to 0, so its Node is dropped.
    // The weak count of 1 from leaf.parent has no bearing on whether or not Node is dropped, so we don’t get any memory leaks!
    println!();
    println!("leaf's parent = {:?}", leaf.parent.borrow().upgrade());  // leaf's parent = None
    println!("leaf's strong and weak counts: {}, {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));  // 1, 0

    println!();
}

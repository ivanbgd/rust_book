#![allow(unused)]

/// PART I - CLOSURES

/// Unlike in the Book, here we check for the number of items that are left for the desired color in the inventory.
/// We use `HashMap` instead of `Vec`.

use std::collections::HashMap;
use std::iter::Iterator;
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ShirtColor {
    Red,
    Blue,
    Yellow,
}

#[derive(Debug)]
struct Inventory {
    shirts: HashMap<ShirtColor, u32>,
}

impl Inventory {
    fn giveaway(&mut self, user_preference: &Option<ShirtColor>) -> Option<ShirtColor> {
        let color = user_preference.unwrap_or_else(|| self.most_stocked());

        if let Some(x) = self.shirts.get_mut(&color) {
            if *x > 0 {
                *x -= 1;
                Some(color)
            } else {
                None
                // self.giveaway(&Some(self.most_stocked()))
            }
        } else {
            panic!("The color {:?} has not been initialized in the inventory!", color);
        }
    }

    fn most_stocked(&self) -> ShirtColor {
        self.shirts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0.clone()
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
    move || s_ref.to_string()  // `move` is not necessary.
}

fn make_a_cloner2(s_ref: &str) -> impl Fn() -> String + '_ {
    || s_ref.to_string()  // `move` is not necessary.
}

fn closures() {
    let mut store = Inventory {
        shirts: HashMap::from([
            (ShirtColor::Red, 1),
            (ShirtColor::Blue, 2),
            (ShirtColor::Yellow, 0),
        ])
    };
    dbg!(&store);

    let mut user_pref = Some(ShirtColor::Yellow);
    let mut giveaway = store.giveaway(&user_pref);
    println!("\nThe user with preference {:?} gets {:?}.", user_pref, giveaway);
    // The user with preference Some(Yellow) gets None.
    dbg!(&store);

    user_pref = Some(ShirtColor::Red);
    giveaway = store.giveaway(&user_pref);
    println!("\nThe user with preference {:?} gets {:?}.", user_pref, giveaway);
    // The user with preference Some(Red) gets Some(Red).
    dbg!(&store);

    // There is at least one Blue shirt available, but they get None instead.
    // They asked for Red, but they should get Blue, and not None.
    // This should be fixed, but that is not the scope of this chapter, so we won't spend time on that.
    // At least the code, as it is, doesn't panic.
    // Perhaps it should be fixed in `Inventory::most_stocked()`.
    user_pref = Some(ShirtColor::Red);
    giveaway = store.giveaway(&user_pref);
    println!("\nThe user with preference {:?} gets {:?}.", user_pref, giveaway);
    // The user with preference Some(Red) gets None.
    dbg!(&store);

    user_pref = None;
    giveaway = store.giveaway(&user_pref);
    println!("\nThe user with preference {:?} gets {:?}.", user_pref, giveaway);
    // The user with preference None gets Some(Blue).
    dbg!(&store);

    user_pref = Some(ShirtColor::Yellow);
    giveaway = store.giveaway(&user_pref);
    println!("\nThe user with preference {:?} gets {:?}.", user_pref, giveaway);
    // The user with preference Some(Yellow) gets None.
    dbg!(&store);

    user_pref = Some(ShirtColor::Yellow);
    giveaway = store.giveaway(&user_pref);
    println!("\nThe user with preference {:?} gets {:?}.", user_pref, giveaway);
    // The user with preference Some(Yellow) gets None.
    dbg!(&store);

    user_pref = None;
    giveaway = store.giveaway(&user_pref);
    println!("\nThe user with preference {:?} gets {:?}.", user_pref, giveaway);
    // The user with preference None gets Some(Blue).
    dbg!(&store);

    user_pref = Some(ShirtColor::Red);
    giveaway = store.giveaway(&user_pref);
    println!("\nThe user with preference {:?} gets {:?}.", user_pref, giveaway);
    // The user with preference Some(Red) gets None.
    dbg!(&store);

    user_pref = None;
    giveaway = store.giveaway(&user_pref);
    println!("\nThe user with preference {:?} gets {:?}.", user_pref, giveaway);
    // The user with preference None gets None.
    dbg!(&store);

    let expensive_closure = |num: u32| -> u32 {
        println!("\nCalculating slowly...");
        thread::sleep(Duration::from_millis(300));
        num
    };
    println!("{}", expensive_closure(101));

    fn add_one_v1(x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };  // i32
    let add_one_v4 = |x| x + 1;  // i32
    println!("\n{} {} {} {}", add_one_v1(7), add_one_v2(7), add_one_v3(7), add_one_v4(7));

    println!();
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    println!();
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    println!();
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list)).join().expect("Expected to join thread.");
    // println!("Trying to use list after the closure is defined: {:?}", list);  // error[E0382]: borrow of moved value: `list`
    println!("After join.");  // Joining is necessary for correct operation.
    // println!("Trying to use list after the closure is defined: {:?}", list);  // error[E0382]: borrow of moved value: `list`

    println!();
    let mut rect_list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    rect_list.sort_by_key(|r| r.width);
    println!("{:#?}", rect_list);

    println!();
    let mut rect_list = [
        Rectangle { width: 1, height: 10 },
        Rectangle { width: 5, height: 3 },
        Rectangle { width: 12, height: 7 },
    ];
    let mut sort_operations = vec![];
    let value = 10; // String::from("by key called");  // Doesn't work with strings, because they don't implement the `Copy` trait.
    // We are trying to count the number of times `sort_by_key` gets called when sorting `rect_list`, in a contrived/convoluted way.
    rect_list.sort_by_key(|r| {
        sort_operations.push(value);
        r.height
    });
    println!("{:#?}", rect_list);
    println!("{:?}; {}", sort_operations, sort_operations.len());  // [10, 10, 10, 10, 10, 10]; 6
    // Why is it called 6 times and not 3 times, as we have 3 elements in the list?

    println!();
    let mut rect_list = [
        Rectangle { width: 1, height: 10 },
        Rectangle { width: 5, height: 3 },
        Rectangle { width: 12, height: 7 },
    ];
    let mut num_sort_operations = 0;
    // We are trying to count the number of times `sort_by_key` gets called when sorting `rect_list`, in a contrived/convoluted way.
    rect_list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.height
    });
    println!("{:#?}", rect_list);
    println!("Sorted in {} operations.", num_sort_operations);  // Sorted in 6 operations. - Why 6 and not 3?

    println!();
    let s_own = String::from("Hello, world!");
    let cloner = make_a_cloner(&s_own);
    // drop(s_own);
    println!("{}", cloner());

    println!();
    let s_own = String::from("Hello, world!");
    let cloner2 = make_a_cloner2(&s_own);
    // drop(s_own);
    println!("{}", cloner2());
}


/// PART II - ITERATORS

pub trait _Iterator {
    type _Item;  // an associated type for this trait

    fn next(&mut self) -> Option<Self::_Item>;

    // methods with default implementations elided
}

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| shoe.size == shoe_size).collect()
}

fn test_filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker"), },
        Shoe { size: 13, style: String::from("sandal"), },
        Shoe { size: 10, style: String::from("boot"), },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker"), },
            Shoe { size: 10, style: String::from("boot"), },
        ]
    );
}

fn audio_decoder() {
    let buffer: &mut [i32] = todo!();
    let coefficients: [i64; 12];
    let qlp_shift: i16;

    println!("{:?}\n{:?}", buffer, coefficients);

    for i in 12..buffer.len() {
        let prediction = coefficients.iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>()
            >> qlp_shift;
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }
}

fn iterators() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for elt in v1_iter {  // &i32
        println!("Got: {}", elt);
    }

    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let v1_iter = v1.iter();
    let total = v1_iter.sum::<i32>();
    assert_eq!(total, 6);
    println!("{}", total);  // 6

    println!("{:?}", v1.iter().map(|x| x + 1));  // Map { iter: Iter([1, 2, 3]) }
    println!("{:?}", v1.iter().map(|x| x + 1).collect::<Vec<i32>>());  // [2, 3, 4]

    println!();
    test_filters_by_size();
}


/// MAIN

fn main() {
    // closures();

    iterators();
}

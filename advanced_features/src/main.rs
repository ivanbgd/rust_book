mod unsafe_rust {
    use std::slice;

    pub fn func1() {
        println!();
        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        // Creating a raw pointer to an arbitrary memory address
        let address = 0x012345usize;
        let _r = address as *const i32;

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
            // println!("_r is: {}", *r);  // STATUS_ACCESS_VIOLATION
        }
    }

    pub unsafe fn dangerous() {}

    pub fn library_split_at_mut_example() {
        println!();
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];
        let (a, b) = r.split_at_mut(3);
        println!("a = {a:?}");
        println!("b = {b:?}");
    }

    fn my_split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();
        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    pub fn my_split_at_mut_example() {
        println!();
        let mut vector = vec![1, 2, 3, 4, 5, 6];
        let (a, b) = my_split_at_mut(&mut vector, 3);
        println!("a = {a:?}");
        println!("b = {b:?}");
    }

    pub fn really_unsafe_fn() {
        println!();
        let address = 0x01234usize;
        let ptr = address as *mut i32;

        // We can create this pointer, with arbitrary length. Creating it doesn't panic.
        let values: &[i32] = unsafe {
            slice::from_raw_parts_mut(ptr, 0)
        };

        // But using it does panic, with STATUS_ACCESS_VIOLATION for anything other than len = 0 above.
        println!("values = {:?}", values);  // []
    }

    extern "C" {
        fn abs(input: i32) -> i32;
    }

    pub fn use_c_abs() {
        println!();
        unsafe {
            println!("Absolute value of -3 according to C is: {}", abs(-3));
        }
    }

    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!();
        println!("Just called a Rust function from C!");
    }

    pub static HELLO_WORLD: &str = "Hello, world!";
    pub static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    pub fn test_add_to_count() {
        println!();
        add_to_count(3);
        unsafe {
            println!("COUNTER = {}", COUNTER);
        }
    }

    pub unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }

    /// This function is dangerous!
    /// It compiles correctly and executes without issue because Vec has enough capacity
    /// such that v.push(4) does not resize it.
    /// However, if the capacity were 3, then n could point to deallocated memory,
    /// because of a possible deallocation.
    pub fn quiz2() {
        println!();
        let mut v = Vec::with_capacity(4);
        for i in 0 .. 3 {
            v.push(i);
        }
        let n = &v[0] as *const i32;
        v.push(4);
        println!("{}", unsafe { *n });
    }
}

mod advanced_traits {
    trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter {
                count: 0,
            }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    pub fn test_counter_with_iterator() {
        println!();
        let mut counter = Counter::new();
        for _ in 0..10 {
            counter.next();
        }
        println!("count = {}", counter.count);
    }

    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Self) -> Self::Output {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    pub fn test_point_add() {
        println!();
        assert_eq!(
            Point { x: -5, y: 3 } + Point { x: 17, y: -10 },
            Point { x: 12, y: -7 }
        );
    }

    #[derive(Clone, Debug, PartialEq)]
    struct Millimeters(u32);

    #[derive(Clone, Debug)]
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, rhs: Meters) -> Millimeters {
            Millimeters(self.0 + rhs.0 * 1000)
        }
    }

    pub fn test_add_meters_to_mm() {
        println!();
        let mil = Millimeters(321);
        let met = Meters(4);
        let res = mil.clone().add(met.clone());  // Cloning only for printing.
        assert_eq!(res, Millimeters(4321));
        println!("Millimeters(321) + Meters(4) = Millimeters(4321): {:?} + {:?} = {:?}", mil, met, res);
        println!("Millimeters(321) + Meters(4) = Millimeters(4321): {} + {} * 1000 = {}", mil.0, met.0, res.0);
        let res = mil + met;
        assert_eq!(res, Millimeters(4321));
    }

    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    pub fn test_fly() {
        println!();
        let person = Human;
        person.fly();
        println!();
        Pilot::fly(&person);
        Wizard::fly(&person);
        person.fly();
        Human::fly(&person);
    }

    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Džeki")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    pub fn test_animal() {
        println!();
        println!("A baby dog is called a {}", Dog::baby_name());  // Džeki
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());  // puppy - this is what we want
    }

    use std::fmt;
    use std::fmt::Formatter;

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point2D {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point2D {}

    pub fn test_supertrait() {
        println!();
        let point = Point2D {
            x: 1,
            y: 3,
        };
        point.outline_print();
        println!();
        Point2D::outline_print(&point);
        println!();
        OutlinePrint::outline_print(&point);
        println!();
        Point2D::outline_print(&Point2D{ x: -34, y: -156 });
    }

    // struct Wrapper<T>(Vec<T>);

    #[derive(Debug)]
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    pub fn test_wrapper() {
        println!();
        let vector = vec![String::from("Hello"), String::from("world!")];
        let wrapper = Wrapper(vector);
        println!("wrapper = {:?}", wrapper);  // wrapper = Wrapper(["Hello", "world!"]) - requires fmt::Debug; this is not what we want
        println!("wrapper = {}", wrapper);  // wrapper = [Hello, world!] - this is our desired output
    }
}

mod advanced_types {
    type Kilometers = i32;

    pub fn test_km() {
        println!();
        let x: i32 = 4;
        let y: Kilometers = 7;
        let z = x + y;
        println!("x + y = {}", z);
    }

    pub fn test_long_type_no_alias() {
        println!();

        let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi!"));

        fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
            f();
        }

        fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
            Box::new(|| ())
        }

        takes_long_type(f);
        let _ = returns_long_type();
    }

    pub fn test_long_type_with_alias() {
        println!();

        type Thunk = Box<dyn Fn() + Send + 'static>;

        let f: Thunk = Box::new(|| println!("bye!"));

        fn takes_long_type(f: Thunk) {
            f();
        }

        fn returns_long_type() -> Thunk {
            Box::new(|| ())
        }

        takes_long_type(f);
        let _ = returns_long_type();
    }

    fn _generic1<T>(_t: T) {}
    fn _generic2<T: Sized>(_t: T) {}
    fn _generic3<T: ?Sized>(_t: &T) {}
}

mod advanced_functions_and_closures {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    /// Using the fn type to accept a function pointer as an argument
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    pub fn test_do_twice() {
        println!();
        let arg = 9;
        let res = do_twice(add_one, arg);
        assert_eq!(20, res);
        println!("res = {}", res);
    }

    pub fn turn_vec_num_into_vec_string1() {
        println!();
        let vec_num = vec![1, 2, 3];
        let vec_string = vec_num.iter().map(|x| (x * 2).to_string()).collect::<Vec<String>>();
        assert_eq!(vec!["2", "4", "6"], vec_string);
        println!("vec_num = {:?}", vec_num);
        println!("vec_string = {:?}", vec_string);
    }

    pub fn turn_vec_num_into_vec_string2() {
        println!();
        let vec_num = vec![1, 2, 3];
        let vec_string = vec_num.iter().map(ToString::to_string).collect::<Vec<String>>();
        assert_eq!(vec!["1", "2", "3"], vec_string);
        println!("vec_num = {:?}", vec_num);
        println!("vec_string = {:?}", vec_string);
    }

    #[derive(Debug)]
    enum Status {
        Value(u32),
        _Stop,
    }

    pub fn initialize_enums() {
        println!();
        let list_of_statuses = (0u32..20).map(Status::Value).collect::<Vec<Status>>();
        println!("list_of_statuses = {:?}", list_of_statuses);  // [Value(0), Value(1), ..., Value(19)]

    }

    fn returns_closure1() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }

    pub fn test_returns_closure1() {
        println!();
        let f = returns_closure1();
        let res = f(7);
        assert_eq!(8, res);
        println!("res = {}", res);
    }

    fn returns_closure2() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    pub fn test_returns_closure2() {
        println!();
        let f = returns_closure2();
        let res = f(7);
        assert_eq!(8, res);
        println!("res = {}", res);
    }

    struct _Event;

    fn _register1(_cb: fn(_Event) -> ()) {}
    fn _register2<F>(_cb: F) where F: Fn(_Event) -> () {}
}

mod macros {
    #[macro_export]
    macro_rules! my_vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    pub fn test_my_vec() {
        println!();
        let v = my_vec![4, 3, 2];
        println!("my_vec v = {:?}", v);
    }

    struct Crepes;

    impl HelloMacro for Crepes {
        fn hello_macro() {
            println!("Hello, Macro! My name is Crepes!");
        }
    }

    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct Pancakes;

    pub fn test_hello_macro() {
        println!();
        Crepes::hello_macro();  // Hello, Macro! My name is Crepes!
        Pancakes::hello_macro();  // Hello, Macro! My name is Pancakes!
    }
}

fn main() {
    unsafe_rust::func1();
    unsafe { unsafe_rust::dangerous(); }
    unsafe_rust::library_split_at_mut_example();
    unsafe_rust::my_split_at_mut_example();
    unsafe_rust::really_unsafe_fn();
    unsafe_rust::use_c_abs();  // 3
    unsafe_rust::call_from_c();  // Just called a Rust function from C!
    println!("\nGlobal (static): \"{}\"", unsafe_rust::HELLO_WORLD);
    unsafe_rust::test_add_to_count();
    unsafe_rust::quiz2();

    println!();
    advanced_traits::test_counter_with_iterator();
    advanced_traits::test_point_add();
    advanced_traits::test_add_meters_to_mm();
    advanced_traits::test_fly();
    advanced_traits::test_animal();
    advanced_traits::test_supertrait();
    advanced_traits::test_wrapper();

    println!();
    advanced_types::test_km();
    advanced_types::test_long_type_no_alias();
    advanced_types::test_long_type_with_alias();

    println!();
    advanced_functions_and_closures::test_do_twice();
    advanced_functions_and_closures::turn_vec_num_into_vec_string1();
    advanced_functions_and_closures::turn_vec_num_into_vec_string2();
    advanced_functions_and_closures::initialize_enums();
    advanced_functions_and_closures::test_returns_closure1();
    advanced_functions_and_closures::test_returns_closure2();

    println!();
    macros::test_my_vec();
    macros::test_hello_macro();
}

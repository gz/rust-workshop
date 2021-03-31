#![allow(unused_mut, unused_variables, dead_code)]

fn main() {
    //
    // 1st Recap: Immutable by default
    //
    let a = 10; // A variable
    //a = 12;   // A mutation
    println!("Immutable by default {}!", a); // A macro to print
    
    //
    // 2nd Recap: Ownership, no aliasing (affine types)!
    //
    let s1 = String::from("No aliasing");
    let s2 = s1;           // NOTE: s1 moved here
    //println!("{}", s1);  // ERROR: use of moved value
    println!("{}", s2);    // OK
    
    //
    // 3rd Recap: Rust Borrowing rules
    //
    // First, you may have one or the other of these two kinds of borrows, 
    // but not both at the same time:
    //
    // - one or more references (&T) to a resource,
    // - exactly one mutable reference (&mut T).
    let a = String::from("hi");
    let mut still_a = print_and_back(a);
    // We can't do this (thanks ownership :/):
    //let mut still_a = print_and_back(a);
    
    // We can do this (thanks borrowing :)):
    borrow_and_print(&still_a, &still_a);
    borrow_and_print(&still_a, &still_a);
    
    //let mut b = String::from("hi");
    //mut_borrow_and_append(&mut still_a);
    //mut_borrow_twice_and_append(&mut still_a, &mut b);
    
    fn print_and_back(s: String) -> String {
        println!("{}", s);
        s
    }
    
    fn borrow_and_print(s: &String, s1: &String) {
        println!("{}", s);
    }
    
    fn mut_borrow_and_append(s: &mut String) {
        s.push_str(" and welcome back");
    }

    fn mut_borrow_twice_and_append(s: &mut String, s1: &mut String) {
        s.push_str(" and welcome back");
    }

    /*
    Practice:
    -----------------------------
    rustlings run move_semantics1
    rustlings run move_semantics2
    rustlings run move_semantics3
    rustlings run move_semantics4
    */
    
    // 
    // 4th Recap: Generics
    //
    // For the real generic Option<T> type, see https://doc.rust-lang.org/std/option/
    enum MyOption<T> {
        MySome(T),
        MyNone
    }
    let m: MyOption<u32> = MyOption::MySome(12);
    let g: MyOption<char> = MyOption::MyNone;
        
    // We can use pattern matching to unpack it
    match g {
        MyOption::MySome(c) => println!("got a character: {}", c),
        MyOption::MyNone => println!("no character")
    }
    
    /*
    Practice:
    -----------------------------
    rustlings run generics1
    rustlings run generics2
    rustlings run generics3
    */

    //
    // 5th Recap: Traits (kinda like interfaces) 
    //
    trait Animal {
        // Instance method signatures; these will return a string.
        fn noise(&self) -> &'static str;
    
        fn talk(&self) {
            // Traits can provide default method definitions.
            println!("{}", self.noise());
        }
    }
    
    struct Sheep { wool: usize }
    
    impl Sheep {
        fn new() -> Self {
            Sheep { wool: 10 }
        }
        
        fn shear(&mut self) {
            if self.wool > 0 {
                self.wool -= 1;
            }
        }
    }

    // Implement the `Animal` trait for `Sheep`.
    impl Animal for Sheep {
        fn noise(&self) -> &'static str {
            "baaaaah!"
        }
        
        // Default trait methods can be overridden.
        fn talk(&self) {
            println!("Sheep says {}", self.noise());
        }
    }
    
    let mut dolly: Sheep = Sheep::new();
    dolly.talk();
    dolly.shear();

    /*
    Practice:
    -----------------------------
    rustlings run traits1
    rustlings run traits2
    */
}

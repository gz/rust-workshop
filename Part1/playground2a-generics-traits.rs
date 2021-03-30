#![allow(unused, dead_code)]
//
// Generics
//

// For the real generic Option<T> type, see https://doc.rust-lang.org/std/option/
enum MyOption<T> {
    MySome(T),
    MyNone
}

fn main() {
    let m: MyOption<u32> = MyOption::MySome(12);
    let g: MyOption<char> = MyOption::MyNone;
    
    // We can use pattern matching to unpack it
    match g {
        MyOption::MySome(c) => println!("got character"),
        MyOption::MyNone => println!("no character")
    }
    
    // Alternative try the `if let` syntax:
    if let MyOption::MySome(num) = m {
        println!("number was {}", num);
    }
}

//
// Traits: a collection of methods defined for an unknown type (similar to Java "interfaces")
//

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

trait Animal {
    // Instance method signatures; these will return a string.
    fn noise(&self) -> &'static str;

    fn talk(&self) {
        // Traits can provide default method definitions.
        println!("{}", self.noise());
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

fn trait_main() {
    let mut dolly: Sheep = Sheep::new();
    dolly.talk();
    dolly.shear();
}

fn compare_prints<T: Animal>(t: &T) {
    println!("{}", t.noise());
}

// We can also specify trait bounds using the `where` syntax
fn compare_prints_where<T>(t: &T) 
  where T: Animal
{
    println!("{}", t.noise());
}

// Debug, Display, Drop, Eq, PartialEq, etc. are often encountered traits from the standard library
// Study the API for some of them, e.g.: https://doc.rust-lang.org/std/fmt/trait.Display.html
// or https://doc.rust-lang.org/std/ops/trait.Drop.html

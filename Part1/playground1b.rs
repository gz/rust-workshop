#![allow(unused, dead_code)]

// Rust Ownership rules:
// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

struct MyWrapperString(String);

// A "destructor" for MyWrapperString
impl Drop for MyWrapperString {
    fn drop(&mut self) {
        println!("4: String '{:?}' dropped", self.0);
        // We could also do things like freeing memory here
        // (in fact this is what drop on String will execute eventually)
    }
}

fn main() {
    {
        let mut y: MyWrapperString;
        {
            let x = MyWrapperString(String::from("hello")); // allocate string hello, x is owner of string hello
            y = x; // ownership of x transfer to y
            // unable to access x here
            println!("1: {}", y.0); // OK
        } // x goes out of scope, but "hello" memory is not freed, y owns it
        println!("2: before the ending the scope of y");
        println!("3: {}", y.0); // OK
    } // y goes out of scope, hello is dropped (free'd) again
    
    
    // give()
}


//
// Ownership transfer across function boundaries:
//
fn give() {
    let a = String::from("hi");
    
    let still_a = print_and_back(a);
    // println!("{}", a) // unable to access moved value here
    // but fortunately `print_and_back` returned it again...
    println!("{}", still_a);
    
    take(still_a); // transferred ownership to `take()`
    // now the string hi is gone (was dropped in take())
}

fn print_and_back(s: String) -> String {
    println!("{}", s);
    s
}

fn take(s: String) {
    println!("{}", s);
}

//
// What if I really need "multi-ownership"?
//

// Use reference counting: https://doc.rust-lang.org/std/rc/index.html
fn reference_counting() {
    use std::rc::Rc;
    let foo = Rc::new(vec![1.0, 2.0, 3.0]);
    // The two syntaxes below are equivalent.
    let a = foo.clone();
    let b = foo.clone();
    // a and b both point to the same memory location as foo.
}

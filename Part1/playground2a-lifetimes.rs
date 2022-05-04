#![allow(unused, dead_code)]

// Rust Borrowing rules:
//
// First, you may have one or the other of these two kinds of borrows, 
// but not both at the same time:
//
// - one or more references (&T) to a resource,
// - exactly one mutable reference (&mut T).
//
// Second, any borrow must last for a scope no greater than that of the owner. 
//
// [Source: https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html#the-rules]


// Fixing yesterday's example:
fn main() {
    let a = String::from("hi");
    
    let mut still_a = print_and_back(a);
    // We can't do this (thanks ownership :/):
    // let mut still_a = print_and_back(a);
    
    // We can do this (thanks borrowing :)):
    //borrow_and_print(&still_a);
    //borrow_and_print(&still_a);
    
    //mut_borrow_and_append(&mut still_a);
    //mut_borrow_twice_and_append(&mut still_a, &mut still_a);
    
    println!("{}", still_a);
    //println!("{}", still_a); // No error of moved value???
}

fn print_and_back(s: String) -> String {
    println!("{}", s);
    s
}

fn borrow_and_print(s: &String) {
    println!("{}", s);
}

fn mut_borrow_and_append(s: &mut String) {
    s.push_str(" and welcome back");
}

fn mut_borrow_twice_and_append(s1: &mut String, s2: &mut String) {
    s1.push_str(" and welcome back");
    s2.push_str(", and again");
}

//
// Second, any borrow must last for a scope no greater than that of the owner. 
//

/*
fn pass_x(x: &i32, y: &i32) -> &i32 { x }
//fn pass_x<'a>(x: &'a i32, y: &'a i32) -> &'a i32 { x }

fn lifetime_main(yref: &mut &i32) {
    let x = 7;
    let y = 9;
    
    let z = pass_x(&x, &y);
}
*/

// Fortunately in most cases we don't have to annotate lifetimes in our program:
// Lifetime elision rules: https://doc.rust-lang.org/nomicon/lifetime-elision.html

// Note aside: Borrowing is the same for 'self' in structs:
struct User {
    active: bool
}

impl User {
    fn activate(&mut self) {
        self.active = true;
    }
    
    fn activate_once(mut self) {
        self.active = true;
    }
}

fn make_user_active() {
    let mut u = User { active: false };
    u.activate();
    u.activate();
    u.activate_once();
    // That doesn't work anymore (`u` moved
    // when we called activate_once):
    // u.activate(); 
}


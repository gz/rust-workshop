#![allow(unused, dead_code)]

// A function
fn main() {
    let a = 10; // A variable
    println!("Hello world {}!", a); // A macro (! at the end) to print
}

//
// Basic data-types
//
fn basic_types() {
    let a: u8 = 1 + 4;
    let b: i64 = -3 * 2;
    let c: bool = true;
    let d: char = 'a';
    // Can you guess some more types?
    //let big: u128 = 0xffff_ffff_ffff_ffff_ffff_ffff;
    //type elision?
}

    
fn compound_types() {
    let mytuple: (i32, f64, char) = (500, 6.4, 'b');
    let myarray: [i32; 5] = [1, 2, 3, 4, 5];
    let mystring1: &'static str = "A string literal";
    let mystring2: String = String::from("A dynamic (heap allocated) String"); // see also https://doc.rust-lang.org/std/string/struct.String.html

    // tuple access?
    // mutability?
}

//
// Struct
//

#[derive(Debug)]
struct User {
    username: &'static str,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn is_active(&self) -> bool {
        self.active
    }
}

struct UserInlined(&'static str, u64, bool);

fn structs() {
    let user = User {
        username: "gerd",
        sign_in_count: 1,
        active: false
    };
    assert!(!user.is_active());
    
    let inlined_user = UserInlined("gerd", 1, false);
    
    // What about dynamic strings?
    // What is #[derive(Debug)]?
}

//
// Enums
//

fn enums() {
    enum IpAddressKind {
        V4,
        V6,
    }
    
    enum IpAddress {
        V4(u32),
        V6(u128),
    }
    
    let ipkind = IpAddressKind::V4;
    let ip = IpAddress::V4(0x0);
    // pattern-matching:
}

//
// Testing
//

// A module that contains unit tests
mod tests {
    #[test]
    fn user_signed_in() {
        let inlined_user = super::UserInlined("gerd", 1, false);
        assert_eq!(inlined_user.0, "gerd");
    }
    
    #[test]
    #[should_panic]
    fn a_failing_test() {
        panic!("This test fails on purpose");
    }
}

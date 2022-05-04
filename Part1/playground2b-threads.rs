#![allow(unused, dead_code)]

// Learning about closures (anonymous functions that can
// capture their environment)
fn main() {
    // Increment an i32 using a closure:
    let closure = |i: i32| -> i32 { i + 1 };

    let i = 1;
    // Call the closures.
    println!("closure: {}", closure(i));
    
    main2();
    //main3();
}

// Learning about Threads + Arc + Mutex
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main2() {
    let numbers: Vec<u32> = (0..100u32).collect();

    let t = thread::spawn(|| {
        let mut i = 0;
        let mut sum = 0;
        while i < numbers.len() {
            sum += numbers[i];
            i += 1;
        }
        println!("Sum on t1 is {}", sum);
    });
    
    /*//let numbers2 = numbers.clone();
    let t2 = thread::spawn(move || {
        let mut i = 0;
        let mut sum = 0;
        while i < numbers.len() {
            sum += numbers[i];
            i += 1;
        }
        println!("Sum on t2 is {}", sum);
    });*/
    
    t.join().unwrap();
    //t2.join().unwrap();
}

// What if you want to mutate the Vector too?
// Use a Mutex https://doc.rust-lang.org/std/sync/struct.Mutex.html
fn main3() {
    let numbers: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new((0..100u32).collect()));

    let t = thread::spawn(move || {
        let mut i = 0;
        let mut sum = 0;
        
        let mut numbers_locked = numbers.lock().unwrap();
        numbers_locked[0] = 99;
    } // The lock is automatically unlocked again here
    // thanks to `numbers_locked` being dropped (scope ends)
    // Sometimes it is helpful to introduce an
    // additional scope ({}) in a large function
    // to narrow down the time a resource is locked
    );

    t.join().unwrap();
}


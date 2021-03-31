// threads4.rs
// Execute `rustlings hint threads4` for hints :)

// I AM NOT DONE

use std::sync::mpsc;
use std::thread;


// Our final `map_with_threads` will be a more realistic version of a parallel map()
// -- similar to rust's amazing data-parallel library rayon.
// This version will also teach us about an important limitation of `std::thread`.
//
// Why is this one more realistic?
//
// - threads2.rs really wasn't all that great because we don't want to always wrap
// every element in a Mutex (when the threads can just work on independent regions).
//
// - threads3.rs copies every element by sending it to a worker thread and then
// copies it again when sending it back. That's pretty inefficient too...
//
// Ideally, we would just split the vector into multiple mutable parts,
// and then send it to worker threads for processing which update the
// elements in-place. No Mutex/Arc, and no copies, great!
//
fn map_with_threads<F>(mut input: Vec<usize>, fun: F) -> Vec<usize> 
where
    F: Copy + Send + 'static,
    F: Fn(usize) -> usize
{
    // We'll use two workers for simplicity, so we split input in the middle:
    let mid = input.len() / 2;
    // The splitting of a vector is well supported: `split_at_mut` returns
    // two independent mutable references to the first and second half of `input`
    let (lo, hi): (&mut [usize], &mut [usize]) = input.split_at_mut(mid);
    
    // Next, we would like to give the mutable `lo` and `hi` slices to one thread each
    // which then apply `fun` on the elements and store the result
    // in place (the slice is mutable after all).
    //
    // The compiler tells us we can't do this?
    // Do you understand why? Use rustlings hint threads4 for an explanation.
    //
    // Unfortunately, this problem can't be solved with `std::thread`.
    // However, there is an alternative threading library from crossbeam 
    // The import is currently uncommented above: `use crossbeam::thread;`
    //
    // Read about it in the docs: https://docs.rs/crossbeam/0.8.0/crossbeam/thread/index.html
    // to see if you can use it to get the code to work:
    //
    let h1 = std::thread::spawn(move || {
        for e in lo {
            *e = fun(*e);
        }
    });
    let h2 = std::thread::spawn(move || {
        for e in hi {
            *e = fun(*e);
        }
    });
    h1.join().unwrap();
    h2.join().unwrap();


    // We return the (now hopefully modified) input as our output
    input
}


fn main() {
    let elements = vec![1,2,3,4];
    // We use the (single-threaded) map() function from the standard library to 
    // generate the expected result for us:
    let expected_result = elements.iter().map(|x| x*2).collect::<Vec<usize>>();
    assert_eq!(map_with_threads(elements, |x| x*2), expected_result);
}
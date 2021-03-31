use crossbeam::thread;

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
    // The import is currently un-commented above: `use crossbeam::thread;`
    //
    // Read about it in the docs: https://docs.rs/crossbeam/0.8.0/crossbeam/thread/index.html
    // to see if you can use it to get the code to work:
    //
    thread::scope(|s| {
        s.spawn(move |_| {
            for e in lo {
                *e = *e*2;
            }
        });
        s.spawn(move |_| {
            for e in hi {
                *e = *e*2;
            }
        });
    }).unwrap();


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
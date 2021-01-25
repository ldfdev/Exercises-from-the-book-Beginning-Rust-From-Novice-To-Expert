
fn main() {

    //heap allocation with Box struct

    let _heap_allocated_num: Box<i64> = Box::new(56);

    const ARRAY_SIZE: usize = 100;
    let _heap_allocated_array: Box<[i64; ARRAY_SIZE]> = Box::new([123_456i64; ARRAY_SIZE]);

    //Box::new returns an address (reference) where the heap allocated object is stored
    println!("Number allocated on heap {} {}", *_heap_allocated_num, _heap_allocated_num);

    // in the second {} above, automatic dereferencing is performed by Rust compiler

    println!("Array allocated on heap {:?} {:?}", *_heap_allocated_array, _heap_allocated_array)

}

fn main() {

    //heap allocation with Box struct

    let _heap_allocated_num: Box<i64> = Box::new(56);
    let _heap_allocated_boolean: Box<bool> = Box::new(true);
    let _heap_allocated_char: Box::<char> =  Box::new('x');

    const ARRAY_SIZE: usize = 100;
    let _heap_allocated_array: Box<[i64; ARRAY_SIZE]> = Box::new([123_456i64; ARRAY_SIZE]);

    // the heap allocated _heap_allocated_array copied from Box::new's function stack

}
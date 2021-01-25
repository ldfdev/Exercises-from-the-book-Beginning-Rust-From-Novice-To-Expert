
fn main() {

    //heap allocation with Box struct

    //since Box::new() returns an address to some heap allocated data,
    // this address can be saved in a reference 
    let _heap_allocated_num = Box::new(23);

    *_heap_allocated_num = 112_113;
    
    let _heap_ref: &mut i32 = &*_heap_allocated_num;

    *&_heap_ref = 112_113;
    // even though _heap+ref is a mutable ref

    println!("{} = {} ?", _heap_allocated_num, _heap_ref)
}

fn main() {

    let _heap_allocated_num = Box::new(23);

    let _heap_ref: &i32 = &*_heap_allocated_num;

    // To know the size in bytes of an object of type T,
    //   use size_of<T>()
    println!("Size of Box holding an i32 {}", std::mem::size_of::<Box<i32>>());
    
    // To know the size in bytes of an object without specifying its type,
    //   use size_of_val(&object)
    println!("Size of such an object address {}", std::mem::size_of_val(_heap_ref));
}
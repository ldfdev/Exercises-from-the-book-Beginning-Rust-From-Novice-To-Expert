
fn main() {

    use std::mem::{size_of, size_of_val};

    println!("Reference type size in bytes for common types. Last 2 columns are the object size and reference size");


    //size_of_val can be used for dynamic types, meaning types whose size is unknown at compile time
    let u_value = 34u64;
    println!("    &u64       containing {:>20} -> {:<5} {}", u_value, size_of_val(&u_value), size_of::<&u64>());

    let c_a = 'a';
    println!("    &char      containing {:>20} -> {:<5} {}", c_a, size_of_val(&c_a), size_of::<&char>());

    let b = true;
    println!("    &bool      containing {:>20} -> {:<5} {}", b, size_of_val(&b), size_of::<&bool>());

    let arr4 = ['a', 'b', 'c', 'd'];
    println!("    &[char; 4] containing {:>20?} -> {:<5} {}", arr4, size_of_val(&arr4), size_of::<&[char; 4]>());
    
    let s: &str = "string";

    // sie_of::<str>() cannot be defined for str
    println!("    &str       containing {:>20} -> {:<5} {}", s, size_of_val(&s), size_of::<&str>());
    
    // all siting literals are stored into static data;
    // this data' size and reference is actually stored in a str object
    println!("The size of a Rust pointer (aka reference) is {} on this machine", size_of::<usize>())
}
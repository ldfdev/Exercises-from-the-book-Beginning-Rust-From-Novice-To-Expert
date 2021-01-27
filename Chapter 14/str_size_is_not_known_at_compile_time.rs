
fn main() {

    let _a: &str = "My cat has no fur";

    // In Rust str objects are stored in static data section
    // varables referring str objects
    //    are special references, holding an address of that object and its size
    
    
    static _B: str = "meaningless";

    // if a variable is instantiated as a str, not reference
    // error[E0277]: the size for values of type `str` cannot be known at compilation time
 
    
}
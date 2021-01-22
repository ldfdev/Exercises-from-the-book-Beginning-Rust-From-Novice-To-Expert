
fn main() {
    // Rust constants are known at compile time
    // compiler replaces any occurence of the constant name with its value
    
    // Even immutable varables are bind at runtime
    
    // constant start in uppercase, by best practices
    const N: usize = 10;
    let _: Vec<i32> = vec![12i32; N];
}
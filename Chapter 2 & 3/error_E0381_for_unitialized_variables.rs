
fn main() {
    // uninitialized mutable and constant variables raise
    // compiler error E0381
    let mut message: String;
    println!("Your welcome mesasge is {}", message);
}
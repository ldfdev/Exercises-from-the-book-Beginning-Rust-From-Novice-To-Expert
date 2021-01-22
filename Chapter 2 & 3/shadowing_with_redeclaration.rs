
fn main() {
    let var = 2.020;
    let mut var = "2.020";
    var = "2019";
    // in Rust redeclaration shadows old variables, making them unaccesible.
    // redeclaration always initializes new variables
    println!("{}", var)
}
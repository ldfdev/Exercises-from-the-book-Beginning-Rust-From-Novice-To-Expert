
fn main() {
    let _: String;
    println!("Unused variables don't produce warnings if they start with _");

    let unused: String;
    println!("Unused variables produce warnings otherwise ");
}
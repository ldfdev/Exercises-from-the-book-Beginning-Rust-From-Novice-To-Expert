
fn main() {
    let mut array = vec![1; 100];
    // replacing avector of int with another vector of int is correct in Rust
    array = vec![2; 200];

    //illegal to change the parameter of vector container fro int to float
    array = vec![1.; 100];
}
    
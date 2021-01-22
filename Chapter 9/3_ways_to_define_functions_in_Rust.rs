
// Rust fuction defined similarly to C++
//   i.e. defined in the global (or some other napespace) scope
fn create_a_pair() -> (i32, i32) {
    (1,2)
}

fn main() {

    create_a_pair();

    fn create_a_triple() -> (i32, i32, i32) {(1,2,3)}
    // Rust accepts functions to be defined inside other functions (defined in their body)

    create_a_triple();


    // Rust allow functions to be invoked first and afterwards declared
    create_a_4_uple();

    fn create_a_4_uple() -> (u32, i32, i32, i32) {(1,2,3,4)};
}
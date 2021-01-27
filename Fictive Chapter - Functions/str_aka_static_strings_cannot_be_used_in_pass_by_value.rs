
// error[E0277]: the size for values of type `str` cannot be known at compilation time
fn compare_with_hidden_string(string: str) -> bool {
    let hidden_string = "Rust is awesome";
    if hidden_string == string {true}
    else {false}
}

fn main() {

    // in rust function sizes must be known at compile time;
    // but for str types their size is not known
    // str is represented as (address, size) which store the address of teh static allocated buffer and its size

    // hence it is only allowed to use reerences to str in function parameters
}
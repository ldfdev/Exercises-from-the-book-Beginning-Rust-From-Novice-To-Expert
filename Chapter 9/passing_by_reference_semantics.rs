
// Passing by reference semantics seem a lot with their C++ counterpart

fn increment_value(input_value: &mut i64) {
    
    // Rust uses the same dereferencing operator as C++, namely *
    *input_value += 1;

}

fn main() {

    let mut a_small_input_value = 8;

    print!("Given {} it becomes ", a_small_input_value);

    increment_value(&mut a_small_input_value);

    println!("{} after being incremented", a_small_input_value);
}
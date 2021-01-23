
// Passing by value copies the value passed

// First copy is made when increment_value is invoked

fn increment_value(mut input_value: i64) -> i64 {
    input_value += 1;

    // Second copy is made when the incremented value is returned
    input_value
}

fn main() {

    let a_small_input_value = 8;

    let incremented_value = increment_value(a_small_input_value);

    println!("Given {} it becomes {} after being incremented", a_small_input_value, incremented_value);
}
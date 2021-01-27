
fn compare_with_hidden_string(string: &str) -> bool {
    let hidden_string = "Rust is awesome";
    if hidden_string == string {true}
    else {false}
}

fn main() {

    let function_pointer: fn(&str) -> bool = compare_with_hidden_string;

    let awesome = "Rust is awesome";
    
    println!("Checking if {}. Response {}", awesome, function_pointer(awesome))
}
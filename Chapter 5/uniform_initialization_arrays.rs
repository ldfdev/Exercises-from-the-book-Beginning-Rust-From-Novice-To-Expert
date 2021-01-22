
fn main() {
    let _5000_elements_array = [0; 5000];

    print!("Array contains {} elements all equal to {}\n", _5000_elements_array.len(), _5000_elements_array[0]);

    for elem in 0.._5000_elements_array.len() {
        print!("{} ", _5000_elements_array[elem])
    }
}
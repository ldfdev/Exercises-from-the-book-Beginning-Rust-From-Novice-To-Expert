
// function generics syntax
// is same as with C++ function templates
fn pick_a_nice_element<T> (elements: &Vec<T>) -> &T {
    
    let mut nice_index: usize = 100;
    while nice_index >= elements.len() {
        nice_index >>= 1;
    }

    let picked = &elements[nice_index];
    picked
}

fn main() {

    let some_elements = vec![12, 13, 14, 15, 200, 201];
    print!("Picking from {:?} a nice element ", some_elements);
    let picked = pick_a_nice_element::<i64>(&some_elements);
    println!("{}", picked)
}
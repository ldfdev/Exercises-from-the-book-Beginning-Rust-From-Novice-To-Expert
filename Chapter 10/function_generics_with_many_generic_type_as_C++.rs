
// function generics syntax
// is same as with C++ function templates
fn pick_index<T> (elements: &Vec<T>) -> usize {
    let mut nice_index: usize = 100;
    while nice_index >= elements.len() {
        nice_index >>= 1;
    }
    nice_index
}

fn pick_a_pair_of_nice_elements<Type1, Type2> (first_batch: &Vec<Type1>, second_batch: &Vec<Type2>) -> (&Type1, &Type2) {

    // on the next 2 lines, the function soecializations for types Type1, respectively Type2
    //   are inferred by the Rust compiler
    let picked_from_first  = &first_batch[pick_index(first_batch)];
    let picked_from_second = &second_batch[pick_index(second_batch)];
    
    (picked_from_first, picked_from_second)
}

fn main() {

    let some_elements = vec![12, 13, 14, 15, 200, 201];
    let another = vec![20, 30, 40, 50, 60, 70, 80, 90, 100];

    println!("Picking a pair of nice elements {:?}", pick_a_pair_of_nice_elements(some_elements, another))
}
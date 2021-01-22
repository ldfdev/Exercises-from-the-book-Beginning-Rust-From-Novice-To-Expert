
fn main() {
    let size = 90;

    let mut array = vec![1; size];
    // vector sizes can be read from variables,
    //  as vector container's size can be modified at runtime

    for _ in 0..array.len() {
        array.push(180);
    }

    println!("Doubling the size to {}", array.len())

    // insert at position 0 a new element
    array.insert(0, 100);

    // remove last element
    array.pop();

    //remove element at position 9
    array.remove(9);

}
    
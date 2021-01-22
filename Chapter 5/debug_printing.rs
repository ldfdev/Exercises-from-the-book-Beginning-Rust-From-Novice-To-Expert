
fn main() {
    
    // debug printing is different than printing

    let v = vec![2112; 5];

    // the following emits error
    //    error[E0277]: `Vec<{integer}>` doesn't implement `std::fmt::Display`
    // println!("Vector contains {}", v);

    println!("Vector contains {:?}", v);
}
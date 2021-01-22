
fn main() {

    let tup: (i32, i32, char) = (10, 20, 'x');
        
    let index: usize = 1;
    const INDEX: usize = 1;

    // tuple fields cannot be accessted with a index variable

    print!("The {}th element of the tuple {:?} is {}", index, tup, tup.index);

    println!("Tuple containing {} {} {}", tup.0, tup.1, tup.2);

}
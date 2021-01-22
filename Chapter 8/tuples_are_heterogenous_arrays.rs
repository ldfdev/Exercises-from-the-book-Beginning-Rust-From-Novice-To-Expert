fn main() {

    // the type of a tuple (similar to arrays) is given by the types of each of its fields
    let tup: (i32, i32, char) = (10, 20, 'x');
    
    // elements of a tuple are called fields
    // fields are addressed to with . notation, like in a struct or classin C++
    println!("Tuple containing {} {} {}", tup.0, tup.1, tup.2);

}

struct S (
    i8,
    i16,
    char,
    bool,
    f64
);

fn main() {



    let s: S = S(1, 2, '3', false, 9.);

    //In Rust a Tuple_Struct is a struct with no named fields

    //addresing elements is same as with tuples
    println!("This tupe struct contains {} {} {} {} {}", s.0, s.1, s.2, s.3, s.4)
    
    // tuple structs are constructed with ()
    // Structs are constructed with {}
}
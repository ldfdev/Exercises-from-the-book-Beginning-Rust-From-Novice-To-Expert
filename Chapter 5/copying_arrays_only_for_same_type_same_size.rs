
fn main() {
    
    let mut v = [2112; 5];
    
    println!("Array contains {:?}", v);

    let _also_5_ints = [19; 5];

    //assignment operator copies rhs into lhs
    v = _also_5_ints;

    println!("Array contains {:?}", v);

    let _exactly_10_ints = [8; 10];

    // error[E0308]: mismatched types
    //   because array is templatised by variable type and array size
    v = _exactly_10_ints

}

fn main() {
    
    let mut v = vec![2112; 5];
    
    println!("Vector contains {:?}", v);

    let b = vec![19; 3];

    //assignment operator copies rhs into lhs
    v = b;

    println!("Vector contains {:?}", v);

}
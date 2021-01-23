
// not specifying a return type in function's signature
//  resumes to having () rturn type

fn create_a_pair() {

    // returning a tuple when compiler infers () type
    //   yields error[E0308]: mismatched types
    (1,2)
}

fn main() {

    
}
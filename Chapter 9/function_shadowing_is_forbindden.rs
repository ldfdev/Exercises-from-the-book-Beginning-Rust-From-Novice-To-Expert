

fn create_a_pair() -> (i32, i32) {
    (1,2)
}

// function shadowind (i.e. redefining) is not allowed
//   as long as they refer to the same scope
fn create_a_pair() -> (i32, i32) {
    (10, 20)
}

fn main() {


    // due to function shadowing
    // compiler issues: error E0428
    //  A type or module has been defined more than once
    create_a_pair()
    
}
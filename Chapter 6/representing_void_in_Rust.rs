
fn main() {
    
    // void is of type ()
    // the only value that () type defines is also written as ()
    let void: () = ();

    // the empty block {} is evaluated as {}
    let void = {};

    // if else clause is missing, else {} is assumed    
    let void = if false {};

    // after ; the current block contains no other statement
    //  so evaluation is understood as ()
    let void = while true {let _ = 0i32;break;};

    println!("Rust void type is {:?}", void)
}

fn main() {
    
    // generic unconstrained integer type defaults to i32
    //  if other type cannot be inferred

    let a = 0;

    let b = 0x70_00_00_00;

    println!("Generic unconstrained integers are inferred as i32: 0x70 00 00 00 = {}", b);

    // if the integer cannot be stored exactly as an i32, error is thrown
    // error: literal out of range for i32

    // let b = 0xf0_00_00_00;

}
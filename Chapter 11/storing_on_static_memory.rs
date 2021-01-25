
fn main() {

    //static variables get allocated with static keyword
    static STORAGE: i64 = 123_456;

    // by best practices it is unsafe to modify static variables in Rust
    // in safe Rust they are immutable by defaut

    static mut CHANGE_ME: i64 = 123_345;

    //error[E0133]: use of mutable static is unsafe and requires unsafe function or block
    CHANGE_ME = 20_000;
    
}

fn main() {
    // after compilation is done
    // each variable has a concrete, constrained type

    // generic unconstrained integer type
    let a = 0;

    let b : u64 = a;
    // type of a has been inferred as u64

    // let c : i64 = a;
    // type mismatch between u64 and i64
    //  error[E0308]: mismatched types
}
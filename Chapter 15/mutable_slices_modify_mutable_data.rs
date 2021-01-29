
fn display_slice_and_modify_underlying_data(slice: &mut [i64]) {

    for i in 0..slice.len() {
        print!("{} ", slice[i]);
        slice[i] = (i + 1) as i64;
    }
    println!("")
}

fn main() {

    //mutable slices can alter the underlying data only if it is mutable as well
    // otherwise
    //  error[E0596]: cannot borrow `ar1` as mutable, as it is not declared as mutable
    let mut ar1 = [12, 12, 12,13];


    display_slice_and_modify_underlying_data(&mut ar1[..]);

    display_slice_and_modify_underlying_data(&mut ar1[..]);
    
}
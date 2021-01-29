
fn display_slice(slice: &[i64]) {

    for i in 0..slice.len() {
        print!("{} ", slice[i]);
    }
    println!("")
}

fn main() {

    let ar1 = [12, 12,12,13];


    //an empty slice is defined as having bpth ends equal
    display_slice(&ar1[1..1]);


    // working with outliers in slices cause panicking
    // display_slice(&ar1[10..0]);

    //working ith decreasing roder slicing also cause panicking
    display_slice(&ar1[2..0]);

}
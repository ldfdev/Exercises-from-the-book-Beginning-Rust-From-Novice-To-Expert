
fn display_slice_as_bytes(slice: &[u8]) {

    for i in 0..slice.len() {
        print!("{} ", slice[i]);
    }
    println!("")
}

fn main() {

    let words = "galactic sunset";
    display_slice_as_bytes(&words.as_bytes());

    let cashflows = [12.098, 190.09, 8876.999];
    display_slice_as_bytes(&cashflows.as_bytes());

    let incomes = vec![1, 2, 3, 4];
    display_slice_as_bytes(&incomes.as_bytes());
}
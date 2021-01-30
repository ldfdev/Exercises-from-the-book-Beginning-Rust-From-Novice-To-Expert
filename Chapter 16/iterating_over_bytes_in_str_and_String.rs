
fn display_slice_as_bytes(slice: &[u8]) {

    for i in 0..slice.len() {
        print!("{} ", slice[i]);
    }
    println!("")
}

fn main() {

    let words = "galactic sunset";
    display_slice_as_bytes(&words.as_bytes());

    let dynamic_words = String::from(words);

    display_slice_as_bytes(&dynamic_words.as_bytes());

}
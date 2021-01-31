


fn display_by_iterating(slice: &[i64; 5]) {

    // iterators for arrays type are accessed via iter() function
    let mut iter = slice.iter();
    loop {
        match iter.next() {
            Some(value) => {
                print!("Curent value {}.\n", value);
            },
            None => {
                println!("");
                break;
            }
        }
        
    }
}

fn main() {

    let contains = [135, 246, 357, 468, 579];
    display_by_iterating(&contains);

}
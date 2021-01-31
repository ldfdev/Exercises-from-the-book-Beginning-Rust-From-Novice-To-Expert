


fn display_by_iterating(slice: &Vec<i64>) {

    // iterators for Vec type are accessed via iter() function
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

    let contains = vec![135, 246, 357, 468, 579];
    display_by_iterating(&contains);

}
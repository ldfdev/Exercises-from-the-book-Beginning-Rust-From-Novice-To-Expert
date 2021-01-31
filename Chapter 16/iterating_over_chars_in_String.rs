
fn display_by_iterating(slice: &String) {

    // iterators for String type are accessed via chars() function
    let mut iter = slice.chars();
    loop {
        match iter.next() {
            Some(value) => {
                print!("Curent value {}. Remaining {}\n", value, iter.as_str());
            },
            None => {
                println!("");
                break;
            }
        }
        
    }
}

fn main() {

    let words = String::from("galactic sunset");
    display_by_iterating(&words);

}
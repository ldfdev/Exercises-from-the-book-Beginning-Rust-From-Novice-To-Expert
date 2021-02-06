

fn main() {

    let mut arr_contains = [135, 246, 357, 468, 579];

    //std::iter::filter excludes elements for wrhich predicate returns false
    let mut odd_numbers_in_array = arr_contains.iter().filter(|x| {**x % 2 == 0});
    println!("FIltering odd numbers from {:?}", arr_contains);

    loop {
        match odd_numbers_in_array.next() {
            Some(value) => print!("{} ", value),
            None => { println!(""); break;}
        }
    }
}
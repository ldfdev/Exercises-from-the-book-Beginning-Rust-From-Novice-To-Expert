

fn main() {

    let mut arr_contains = [135, 246, 357, 468, 579];

    let mut odd_numbers_in_array = arr_contains.iter().map(|x| {*x + (*x % 2)});
    println!("Incrementing odd numbers from {:?}", arr_contains);

    loop {
        match odd_numbers_in_array.next() {
            Some(value) => print!("{} ", value),
            None => { println!(""); break;}
        }
    }
}
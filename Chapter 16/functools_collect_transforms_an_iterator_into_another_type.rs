

fn main() {

    let  arr_contains = [135, 246, 357, 468, 579];

    let transformed = arr_contains.iter().map(|x| {*x + (*x % 2)}).collect::<Vec<_>>();
    // or
    let transformed: Vec<i64> = arr_contains.iter().map(|x| {*x + (*x % 2)}).collect();
    println!("Incrementing odd numbers from {:?} into {:?}", arr_contains, transformed);
}
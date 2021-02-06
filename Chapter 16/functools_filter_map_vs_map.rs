
fn count_digits(mut num: i64) -> Result<String, String> {
    let mut digits = 0;
    while num > 0 {
        digits += 1;
        num = num / 10 as i64;
    }
    if digits > 3 {
        Ok(String::from("more than 3"))
    } else if digits > 1 {
        Ok(String::from("regular"))
    } else {
        Err(format!("Too few digits {}", digits))
    }
}

fn main() {

    let  arr_contains = [135, 2460, 357, 46812, 57923, 12, 0, 9, 7, 3];
    
    // map always expects as returned type  Option<T>.
    // ok() from Result converts an Result<T, E> into an Otion<T>

    // filter keeps only values wrapped with Some()

    let filter_map_collected: Vec<String> = arr_contains.iter().filter_map(|&x| count_digits(x).ok() ).collect();

    println!("filter from filter_map keeps only values wrapped with Some() \n\t{:?}", filter_map_collected);

    let map_collected: Vec<Option<String>> = arr_contains.iter().map(|&x| count_digits(x).ok() ).collect();

    println!("map keeps all values \n\t{:?}", map_collected);

    }
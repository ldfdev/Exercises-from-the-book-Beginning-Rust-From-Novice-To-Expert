
fn main() {
    
    let _true = true;
    println!("{} as u8 {}", _true, _true as u8);

    // numeric types are forbidden to cast to bool
    // error E0054
    // println!("1 as u8 as bool {}", 1u8 as bool);

    // println!("101 as u8 as bool {}", 101u8 as bool);


}
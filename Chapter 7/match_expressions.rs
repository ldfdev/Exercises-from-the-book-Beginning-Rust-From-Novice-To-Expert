
fn main() {

    let var = 9u8;

    // the construction _ if
    //  is called guard

    // in the match expression
    //  all arms (branches) must return same type

    println!("{} is {}", var, 
        match var {
            _ if var < 0 => "less than zero",
            _ if var < 5 => "small number less than 5",
            _ => "A reasonable number"
        })
    
}
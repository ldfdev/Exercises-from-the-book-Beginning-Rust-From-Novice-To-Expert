
#[derive(Debug)]
struct DuckDescription {
    duck_name: Vec<char>,
    duck_age_in_months: u16,
    duck_feather_color: char,
    is_duck_quacking_loudly: bool
}

fn main() {

    // In Rust structs are tuples whose fields are named
    // thus in a struct fields are reffered to by their names

    let duckling1 = DuckDescription {
        duck_name: vec!['a','l','i','c','e'],
        duck_age_in_months: 19,
        duck_feather_color: 'w',
        is_duck_quacking_loudly: true
    };

    println!("The first duckling recorded data {:?}", duckling1);

    print!("{}", duckling1.0);
}
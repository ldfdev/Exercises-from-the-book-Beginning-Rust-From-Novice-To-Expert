
struct Pair <T1, T2> {
    first: T1,
    second: T2
}

fn main() {

    // when constructing a struct instance In rust we must specify the fields names as well
    //   below Pair::<char, char>{'a', 'b'}
    //   emits compiler error E0063
    //   A struct's or struct-like enum variant's field was not provided.
    
    let pair = Pair::<char, char>{first: 'a', second: 'b'};

    println!("Picking the pair <{} {}>", pair.first, pair.second)
}
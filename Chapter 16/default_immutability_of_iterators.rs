
//uniform iteration for container supporting iterators

fn display_by_iterating_in_arrays(mut slice: &[i64; 5]) {

    //or slice.iter() below
    for &mut mut_ref in slice {
        // iterating arrays yields immutable iterator by default
        // error[E0308]: mismatched types
        // |         types differ in mutability
        // |
        // = note:   expected reference `&i64`
        //           found mutable reference `&mut _`
        //  mut_ref = 1e7 as i64 + mut_ref;
        print!("Curent value {}.\n", mut_ref);    
    }
}

fn display_by_iterating_in_vec(slice: &Vec<i64>) {

    // calling .iter() is optional for vectors
    
    //same immutabilty by default
    for &mut value in slice {
        print!("Curent value {}.\n", value);    
    }
}

fn display_by_iterating_in_slices(slice: &[i64]) {

    // calling .iter() is optional for slices

    //same immutability by default
    for &mut value in slice.iter() {
        print!("Curent value {}.\n", value);    
    }
}

fn display_by_iterating_in_string(slice: &String) {

    // calling .chars() is mandatory for dynamic strings
    
    //same immutability by default
    for &mut value in slice.chars() {
        print!("Curent value {}.\n", value);    
    }
}

fn display_by_iterating_in_str(slice: &str) {

    // calling .chars() is mandatory for static strings
    
    //same immutability by default
    for &mut value in slice.chars() {
        print!("Curent value {}.\n", value);    
    }
}

fn main() {

    let arr_contains = [135, 246, 357, 468, 579];
    display_by_iterating_in_arrays(&arr_contains);

    let vec_contains = vec![10, 100, 1000, 10_000, 100_000];
    display_by_iterating_in_vec(&vec_contains);

    let slice: &[i64] = &vec_contains[..];
    display_by_iterating_in_slices(slice);

    let string_contains = String::from("Learning Rust is no harder than learning chinese");
    display_by_iterating_in_string(&string_contains);

    let str_contains = "never judge a language only by its syntax";
    display_by_iterating_in_str(&str_contains);
}
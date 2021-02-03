
//uniform iteration for container supporting iterators

fn display_by_iterating_in_arrays(slice: &mut [i64; 5]) {

    // dereferencing is required because iter_mut() reeturns a (mutable) ref to a contained value
    for mut_ref in slice.iter_mut() {
        *mut_ref = 1e7 as i64 + *mut_ref;
    }
    for mut_ref in slice.iter() {
        print!("Curent value {}.\n", mut_ref);    
    }
}

fn display_by_iterating_in_vec(slice: &mut Vec<i64>) {
    
    // dereferencing is required because iter_mut() reeturns a (mutable) ref to a contained value
    for value in slice.iter_mut() {
        *value = 1e5 as i64;    
    }
    for value in slice.iter() {
        print!("Curent value {}.\n", value);    
    }
}

fn display_by_iterating_in_slices(slice: &mut [i64]) {

    // dereferencing is required because iter_mut() reeturns a (mutable) ref to a contained value
    for value in slice.iter_mut() {
        *value = 111_111;
    }
    for value in slice.iter() {
        print!("Curent value {}.\n", value);    
    }
}


fn main() {

    let mut arr_contains = [135, 246, 357, 468, 579];
    display_by_iterating_in_arrays(&mut arr_contains);

    let mut vec_contains = vec![10, 100, 1000, 10_000, 100_000];
    display_by_iterating_in_vec(&mut vec_contains);

    let slice: &mut[i64] = &mut vec_contains;
    display_by_iterating_in_slices(slice);

}
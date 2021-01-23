
fn sum_first_n_elements(upper_limit: u64, mut returned_sum: u64) {
    returned_sum = 0;
    for i in 0..1 + upper_limit {
        returned_sum += i;
    }
    println!("Summing first {} numbers yields {}", upper_limit, returned_sum);
}
fn main() {

    // In Rust function parameters are passed by value by default

    let mut sum_of_numbers = 100;
    let upper_lim = 100;
    
    println!("Guessing that the sum of first {} numbers is {}", upper_lim, sum_of_numbers);
    
    sum_first_n_elements(upper_lim, sum_of_numbers);

    println!("After the computation the sum of first {} numbers is {}", upper_lim, sum_of_numbers);
}
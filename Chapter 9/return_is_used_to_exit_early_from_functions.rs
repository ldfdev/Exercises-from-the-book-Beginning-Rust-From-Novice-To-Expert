
fn sum_first_n_natural_elements(upper_limit: u64) -> u64 {
    if upper_limit <= 0 {
        return 0;
    }

    let mut returned_sum = 0;
    for i in 0..1 + upper_limit {
        returned_sum += i;
    }
    println!("Summing first {} numbers yields {}", upper_limit, returned_sum);
    returned_sum
}
fn main() {

    let upper_lim = 100;
    
    
    let result = sum_first_n_natural_elements(upper_lim);

    println!("After the computation the sum of first {} numbers is {}", upper_lim, result);
}

// in Rust mut in function signatures allow for parameter modification
// if arr below wouldn't have been declared mutable, the compiler issued
//   E0594 A non-mutable value was assigned a value

fn replace_zero_in_array(mut arr: [i64; 5]) -> u64 {
    let replacing_value = -1e3 as i64;
    let mut count_zeros = 0u64;

    for i in 0..arr.len() {
        if arr[i] == 0 {
            count_zeros += 1;
            arr[i] = replacing_value;
        }
    }
    
    println!("Detected {} zeros. Array becomes {:?}", count_zeros, arr);

    count_zeros
}

fn main() {

    let array_with_3_zeros = [1, 0, 2, 0, 0];

    replace_zero_in_array(array_with_3_zeros);
    
}
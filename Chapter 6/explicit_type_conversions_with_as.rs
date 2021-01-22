
fn main() {
    
    let a: [f32; 2] = [1., 9.];

    let mut sum = 0 as i32;

    for i in 0..2 {
        sum += a[i] as i32;
    }

    println!("Sum of {:?} as integer is {}", a, sum);

}
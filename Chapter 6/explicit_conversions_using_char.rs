
fn main() {
    
    let hao: char = '好';
    let a: char = 'a';

    println!("{} as i32 {} as u8 {}\
        {} as i32 {}", hao, hao as i32, hao as u8, a, a as i32);
    println!("Numeric {} as char {} ", hao as u8, (hao as u8) as char);

    // Numeric to char is allowed only from u8
    // otherwise E0604: A cast to `char` was attempted on a type other than `u8`
}
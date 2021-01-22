
fn main() {
    
    let signed_8_bit:  i8  = 127;
    let signed_16_bit: i16 = 127;
    let signed_32_bit: i32 = 127;
    let signed_64_bit: i64 = 1_000_000_000_000;
    
    println!("Signed numeric types \
        signed  8 bit {}\n\
        signed 16 bit {}\n\
        signed 32 bit {}\n\
        signed 64 bit {}\n\
        ", signed_8_bit, signed_16_bit, signed_32_bit, signed_64_bit);
    
    // i8, i16 etc are of different types
    // error[E0308]: mismatched types
    let sum_ = signed_8_bit + signed_16_bit + signed_32_bit + signed_64_bit;
}
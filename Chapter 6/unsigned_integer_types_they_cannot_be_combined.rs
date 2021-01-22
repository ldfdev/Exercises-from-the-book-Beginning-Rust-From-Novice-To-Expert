
fn main() {
    
    let unsigned_8_bit:  u8  = 0xFA;
    let unsigned_16_bit: u16 = 127;
    let unsigned_32_bit: u32 = 0x_A_B_C_D;
    let unsigned_64_bit: u64 = 1_000_000_000_000;
    
    println!("Signed numeric types \
        unsigned  8 bit {}\n\
        unsigned 16 bit {}\n\
        unsigned 32 bit {}\n\
        unsigned 64 bit {}\n\
        ", unsigned_8_bit, unsigned_16_bit, unsigned_32_bit, unsigned_64_bit);
    
    // i8, i16 etc are of different types
    // error[E0308]: mismatched types
    // let sum_ = unsigned_8_bit + unsigned_16_bit + unsigned_32_bit + unsigned_64_bit;
}
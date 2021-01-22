
fn main() {
    
    // doesn't matter if A..F or a..f are used in hexadecimal representation
    let mut hexadecimal = 0xFF_FF;
    hexadecimal = 0xFf;
    hexadecimal = 0xff;

    let octal = 0o11_12;

    let binary = 0b100;

    println!("Hexa {}, Octa {}, bin {}", hexadecimal, octal, binary);

    // however designating a number base must be done 
    //   0x - hexadecimal; not 0X
    //   0o - octal;       not 0O
    //   0b - binary;      not 0B

    //machine code representation is always in binary

}
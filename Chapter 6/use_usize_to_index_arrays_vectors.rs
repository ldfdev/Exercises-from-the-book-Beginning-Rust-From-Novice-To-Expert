
fn main() {
    
    // depending on the target architecture size
    //  currently being supported only 32 or 64 bits wide

    let signed_index:   isize = 3;
    let unsigned_index: usize = 4;

    let container = [12; 100];

    println!("Accesing elements in a container with isize/usize integer types \
        @ index {} is {}",
        unsigned_index,
        container[unsigned_index],
        );
    
    // for performance, isize istranslated into i32/u32 or i64/u64, according to target architecture

    // on 32 bit machines it is used i32/u32
    // on 64 bit machines it is used i64/u64
}
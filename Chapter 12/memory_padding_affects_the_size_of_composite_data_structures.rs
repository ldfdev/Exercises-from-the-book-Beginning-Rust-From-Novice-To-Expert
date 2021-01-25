
fn main() {
    

    enum ChineseYearMonthWeek {Nian(u64), Yue(u32), Zhou};

    println!("memory padding affects the size of composite data structures {} bytes", std::mem::size_of_val(&ChineseYearMonthWeek::Nian(0xFF_FF_FF_AC_FF_FF_FF_AC)));
    println!("{} bytes", std::mem::size_of::<ChineseYearMonthWeek>());

    // when enums variants do not define data,  
    // compiler allocates for such enum objects 1 byte,
    // same as in C++ with structs/classes without fields

}
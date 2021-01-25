
fn main() {
    

    enum ChineseYearMonthWeek {Nian, Yue, Zhou};

    println!("{}", std::mem::size_of_val(&ChineseYearMonthWeek::Zhou));

    // when enums variants do not define data,  
    // compiler allocates for such enum objects 1 byte,
    // same as in C++ with structs/classes without fields

}
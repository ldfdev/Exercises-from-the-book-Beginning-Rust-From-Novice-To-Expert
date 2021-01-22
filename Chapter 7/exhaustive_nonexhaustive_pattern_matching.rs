
fn main() {

    // enum introduces a new type
    // 
    enum DirectionChinese {
        YOU,
        SHANG,
        XIA,
        ZUO
    };

    let chosen_direction: DirectionChinese = DirectionChinese::YOU;

    // non exhaustive pattern matching is flagged at compile time
    //  error[E0004]
    match chosen_direction {
        DirectionChinese::SHANG => print!("SHANG is up in chinese"),
        DirectionChinese::XIA => print!("XIA is up in chinese"),
        DirectionChinese::YOU => print!("YOU is up in chinese"),
        // DirectionChinese::ZUO => print!("ZUO is up in chinese"),
    }

    // exhaustive pattern matching is in place
    match chosen_direction {
        DirectionChinese::SHANG => {
            // print!("SHANG is up in chinese"); is als allowed
             
            print!("SHANG is up in chinese")
        },
        _ => print!("Default")
    }

}
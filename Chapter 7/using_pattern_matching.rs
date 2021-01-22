
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

    // parretn matching is sintactically different if blocks are used
    //  with blocks, ; is optional at the end
    //  without, it is mandatory not to have ;
    match chosen_direction {
        DirectionChinese::SHANG => print!("SHANG is up in chinese"),
        _ => print!("Default")
    }

    match chosen_direction {
        DirectionChinese::SHANG => {
            // print!("SHANG is up in chinese"); is als allowed
             
            print!("SHANG is up in chinese")
        },
        _ => print!("Default")
    }

}

fn main() {
    println!("True and False {}", true && false);
    println!("True or False {}", true || false);
    println!("Not True {}", !true);
    println!("Not False {}", !false);
    
    // logical operators precedence

    // !  - highest
    // && - higher
    // || - lower

    let _: bool;
    
    println!("true || !true {}", true || !true);
    println!("true && false || true {}", true && false || true)
}
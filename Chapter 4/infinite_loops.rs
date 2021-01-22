
fn main() {
    let mut idx = 0;
    while true {
        idx += 1;
        if idx > 1000000 {
            break;
        }
    }

    // Rust denotes infinite loops by loop keywork, which is preferred over while true

    idx = 0;
    loop {
        idx += 1;
        if idx > 1000000 {
            break;
        }
    }
}
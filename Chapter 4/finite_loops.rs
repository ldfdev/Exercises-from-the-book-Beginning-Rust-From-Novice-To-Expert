
fn main() {
    let mut idx = 0;
    while idx < 10 {
        print!("Index value is {}\n", idx);
        idx += 1;
    }

    for mut idx in 1..10 {
        // last range element is always exclusive
        print!("Index value is {}\n", idx);
        idx += 1;
    }
}

fn main() {
    let mut idx = 0;

    for mut idx in 1..10 {
        // last range element is always exclusive
        print!("Index value is {}\n", idx);
        idx += 1;
    }

    // the for loop bounds are always eveluated before the loop is executed
    
    for mut idx in idx + 1..if true {10} else {123} {
        // last range element is always exclusive
        print!("Index value is {}\n", idx);
        idx += 1;
    }
    
}
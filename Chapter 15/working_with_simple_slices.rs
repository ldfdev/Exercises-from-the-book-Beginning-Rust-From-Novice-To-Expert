
fn display_slice(slice: &[i64]) {

    for i in 0..slice.len() {
        print!("{} ", slice[i]);
    }
    println!("")
}

fn main() {

    let ar1 = [12, 12,12,13];
    display_slice(&ar1);

    let ar2 = [132; 10];
    display_slice(&ar2);

    display_slice(&ar2[2..8]);

    let mut ar3 = vec![64; 0];
    ar3.push(9);
    ar3.push(99);
    ar3.push(999);

    display_slice(&ar3)
    
}
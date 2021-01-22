
fn main() {
    let n: i64 = 1e4 as i64;
    let digits = [1,2,3,4];

    match n {
        1_000 => {println!("{} is matched with 1e3", n)},
        _ => {println!("1e3 is not matched with {}", n)}
    }

    match digits {
        [1,2,3,4] => (),
        _ => {
            println!("digits {:?} not matched", digits);
        }
    }

}
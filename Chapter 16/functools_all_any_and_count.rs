
fn main() {

    let arr_contains = [135, 2460, 357, 46812, 57923, 12, 0, 9, 7, 3];
    
    const THRESHOLD: i64 = 100;

    //double reference below because
    //  iter works on references
    //  filter works on references (references to iter references)
    let items_gt_100 = arr_contains.iter().filter(|&&x| x > THRESHOLD).count();

    println!("Values from {:?}\n greater than {}: {}", arr_contains, THRESHOLD, items_gt_100);

    println!("All are greater than {} ? {}", THRESHOLD, arr_contains.iter().all(|&x| x > THRESHOLD));

    println!("Any  is greater than {} ? {}", THRESHOLD, arr_contains.iter().any(|&x| x > THRESHOLD));

    }
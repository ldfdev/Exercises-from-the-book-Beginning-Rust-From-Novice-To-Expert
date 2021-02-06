

fn main() {

    let  arr_contains = [135, 246, 357, 468, 579];
    const TAKEN_ELEMENTS: usize = 1;
    // take and skip as in the Haskell language

    //take yields an iterator with references to original elements
    let head: Vec<i64> = arr_contains.iter().take(TAKEN_ELEMENTS).map(|&x| x).collect();
    
    // take doesn't change the state of original iterator
    // arr_contains.next() points to the first element, i.e. 135 even though the first has been consumed by take
    println!("After taking {} elements, the iterator points to {}", TAKEN_ELEMENTS, **arr_contains.iter().peekable().peek().unwrap());

    let tail: Vec<i64> = arr_contains.iter().skip(TAKEN_ELEMENTS).map(|&x| x).collect();


    println!("Head and tail of {:?} are {:?} and {:?}", arr_contains, head, tail);
}
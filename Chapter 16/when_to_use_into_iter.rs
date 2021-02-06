

fn main() {

    let arr_1 = vec![135, 2460, 357, 46812, 57923, 12, 0, 9, 7, 3];
    
    let arr_2 = (1..11).collect::<Vec<i64>>();

    let mut computations = Vec::<&Vec<i64>>::new();

    computations.push(&arr_1);
    computations.push(&arr_2);

    const THRESHOLD: i64 = 100;


    // into_iter trnsforms &std::Vec<i64>::Iter into std::Vec<i64>::Iter
    // otherwise, by using iter():
    //  error[E0277]: `&&Vec<i64>` is not an iterator
    // into_iter is implemented for some types to obtain an iterator from a value
    
    let items_gt_100 = computations.into_iter().flatten().filter(|&&x| x > THRESHOLD).count();




    println!("Values from \n\t{:?} and from\n\t{:?}\ngreater than {}: {}", arr_1, arr_2, THRESHOLD, items_gt_100);

}

fn stack_computations(computations: &mut Vec<i64>, new_computation: &mut i64) -> () {
    (*computations).push(*new_computation);
}

fn main() {

    let arr_1 = vec![135, 2460, 357, 46812, 57923, 12, 0, 9, 7, 3];



    let mut computations = Vec::<i64>::new();

    for mut value in arr_1 {
        stack_computations(&mut computations, &mut value)
    }
    const THRESHOLD: i64 = 100;


    let items_gt_100 = computations.iter().filter(|x| *x > &THRESHOLD).count();

    println!("Values from \n\t{:?}\ngreater than {}: {}", computations, THRESHOLD, items_gt_100);

}
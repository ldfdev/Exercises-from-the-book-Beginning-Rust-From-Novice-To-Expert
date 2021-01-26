

fn main() {

    const VALUE: i64 = 3e5 as i64;

    // functions are impure w.r.t constants
    
    fn add_value_to_vec(container: &mut Vec<i64>) {
        print!("Changing content from {:?}", container);
        for i in 0..container.len() {
            container[i] += VALUE;
        }
        println!(" to {:?}", container);
    }

    let mut c = vec![10, 20, 30, 40];

    add_value_to_vec(&mut c);

}


fn main() {

    let value = Box::new(3e5 as i64);

    // functions are pure w.r.t heap allocated variables
    //   error[E0434]: can't capture dynamic environment in a fn item

    fn add_value_to_vec(container: &mut Vec<i64>) {
        for i in 0..container.len() {
            container[i] += *value;
        }
    }

    let mut c = vec![10, 20, 30, 40];

    add_value_to_vec(&mut c);

}

fn main() {

    fn add_value_to_vec(container: &mut Vec<i64>) {
        
        print!("Changing content from {:?}", container);
        for i in 0..container.len() {
            // syntax for a lambda taking i64 as arg
            container[i] = (|q: i64| container[i] +  q) (113_13);

            // if the return type is explicit then the body of the lambda sould be enclosed with {}
            //             (|q: i64| -> i64 {container[i] +  q}) (113_13)
        }
        print!(" to {:?}", container);
        
    }

    let mut c = vec![10, 20, 30, 40];

    add_value_to_vec(&mut c);

}
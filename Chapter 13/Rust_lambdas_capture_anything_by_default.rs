
fn main() {

    fn add_value_to_vec(container: &mut Vec<i64>) {
        let value = 3e5 as i64;
        static SUBTRACT_VALUE: i64 = 1e5 as i64;
        let multiply_value: Box<i64> = Box::new(4);

        print!("Changing content from {:?}", container);
        for i in 0..container.len() {
            container[i] += value;
        }
        print!(" to {:?}", container);
        
        // lambdas capture anything by default
        //   including static, stack and heap allocated data

        (|| {
            for i in 0..container.len() {
                container[i] += value;
                container[i] -= SUBTRACT_VALUE;
                container[i] *= *multiply_value;
            }   
        }) ();
        println!(" and finally to {:?}", container);
        
    }

    let mut c = vec![10, 20, 30, 40];

    add_value_to_vec(&mut c);

}

fn judge_quantity (quantity: f64) -> Result<f64, String> {
    match quantity {
        _ if quantity < 10. => Err(format!("Quantity {} is not enough", quantity)),
        _ if quantity < 20. => Err(format!("quantity {} is moderate", quantity)),
        _ => Ok(quantity)
    }
}

fn main() {

    let quantities = vec![2., 3.12, 4., 10., 17., 30.];

    for quantity in quantities {
        let result = judge_quantity(quantity);

        // Instead of matching a Result enum
        // Rust provides the more convenient ways is_ok() and unrap()

        println!("{}", 
            if result.is_ok() {
                result.unwrap()
            }
            else {
                0.
            }
        )
    }
}
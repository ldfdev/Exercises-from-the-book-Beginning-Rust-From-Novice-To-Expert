
fn main() {

    const ERROR_SIZE: usize = 17;
    enum BoxedComputation {
        Result(i32),
        Failure(u8, [char; ERROR_SIZE])
    }

    let result = BoxedComputation::Failure(0xf3, ['s', 'e', 'g', 'm', 'e', 'n', 't', 'a', 't', 'i', 'o', 'n', 'f', 'a', 'u', 'l', 't']);

    match result {
        BoxedComputation::Result(value) => {
            println!("Computation was performed succesfully. Retrieved value {}", value);
        }
        BoxedComputation::Failure(error_code, error_message) => {
            println!("During computation error {} occured. Error description is {:?}", error_code, error_message);
        }
    }
}
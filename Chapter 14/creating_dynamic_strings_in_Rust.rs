
fn main() {

    //creating Sting objects from (static) string literal [1]
    let _a: String = String::from("My cat has no fur");
    
    //creating Sting objects from (static) string literal [2]
    let _a: String = "I am elarning Rust".to_string();

    //creating Sting objects from (static) string literal [3]
    let _a: String = "A third method to comnstruct a String".into();

    //creating empty String [1]
    let _a = String::new();

    //creating empty String [2]
    let _a: String = format!("");


    //creating Sting objects by concatenating
    let _b: String = _a + " . Who knows how many othere are left ?";
    
}
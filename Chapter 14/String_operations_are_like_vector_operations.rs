
fn main() {

    //creating Sting objects from (static) string literal [1]
    let mut _a: String = String::from("My cat has no fur");
    
    //String operations are similar to vector operations

    _a.insert(_a.len() - 1, '!');

    _a.pop(); // equivalent with _a.remove(_a.len() - 1)

    _a.capacity();


    println!("{}", _a);

}
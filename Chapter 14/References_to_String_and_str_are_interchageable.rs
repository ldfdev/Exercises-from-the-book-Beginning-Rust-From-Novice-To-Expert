
fn main() {

    //String references and str references are compatible
    let mut a: String = String::from("My cat has no fur");

    let mut b = String::new();

    // += or push_str requires a ref to str
    //  but here a reference to String is converted to a reference to str
    b += &a;

    // the reverse is also possible;
    //  a reference to Str is used to construct a static string
    let c: &str = &b;
    


}
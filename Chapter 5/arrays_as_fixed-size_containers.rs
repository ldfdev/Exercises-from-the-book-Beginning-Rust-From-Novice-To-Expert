
fn main() {
    // in Rust arrays are generic containers of constant size
    // they are templatised based on contained value type and container size

    let arr = [1., 2., 3.];
    print!("Array length {}; first element {}; last element {}\n", arr.len(), arr[0], arr[arr.len() - 1]);

    let _bad = arr[4];

    // this code compiles but aborts at runtime
    // #[deny(unconditional_panic)] should be disabled to let the code compile and crash at runtime
    // this behaviour is intended by Rust compiler

    // premature termination of a program is called PANIC

    let mut arr = ["Aripa", "Strugurel", "Tutankamon"];

    arr[0] = "Zbanghiul";

    // mutating an array in Rs means altering its content
    //    never increasing nor decreasing its size

    
    // compilation error E0308 as array type changes
    //    from array<String, 3> to array<String, 4>
    arr = ["Aripa", "Strugurel", "Tutankamon", "Soricioaica"];

}
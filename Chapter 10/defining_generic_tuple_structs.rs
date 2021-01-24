
struct UnnamedPair <T1, T2> (
    T1,
    T2
);

fn main() {


    let unnamed_pair = UnnamedPair::<char, char>('a', 'b');
    
    // generic tuple struct specialization can also be inferred by the compiler
    let unnamed_pair = UnnamedPair('a', 'z');

    println!("Picking the pair <{} {}>", unnamed_pair.0, unnamed_pair.1)
}
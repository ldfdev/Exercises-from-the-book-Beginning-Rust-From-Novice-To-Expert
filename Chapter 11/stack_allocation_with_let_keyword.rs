
fn main() {

    //stack allocation is requested with let keyword

    let _num = 56;
    let _truth = true;
    let _special_char = 'x';

    // every thread knows the limit addresses of its stack ends
    // stack is allocated for every thread
    // at the beginning of each function invocation,
    //  the stack pointer is incremented (according to how many stack allocations are needed for function' params and local variables)
    //  then function goes out of scope the reverse work is done
    // local variables' addresses are kept as offsets from the SP

}
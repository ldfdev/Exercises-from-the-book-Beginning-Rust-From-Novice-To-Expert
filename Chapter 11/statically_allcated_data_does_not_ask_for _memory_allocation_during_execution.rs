
fn main() {

    //static variables get allocated with static keyword
    static STORAGE: i64 = 123_456;
    static STORAGE2: i64 = 123_456;
    static STORAGE3: i64 = 123_456;

    let name = "shuo hao means to always say truth";

    //variables allocated in static memory are allocated contiguously
    // string literals a re as well allocated on static memory

    // all static memory is requested by the executable program once 
    //   when it enters execution
    //   no other memory allocations are requested during runtime, considering variables allcated on static memory

}
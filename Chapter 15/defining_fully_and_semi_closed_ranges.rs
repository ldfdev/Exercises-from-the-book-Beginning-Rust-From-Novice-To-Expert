
fn main() {

    //defining semi closed ranges [left, right)

    let mut arr = 1..10;

    //same type, more verbose
    arr = std::ops::Range::<i32> {start: 1, end: 10};

    assert!(!arr.contains(&10));

    //defining fully closed ranges [let, right]


    let mut arr2 = 1..=10;
    
    //same type, more verbose
    arr2 = std::ops::RangeInclusive::<i32>::new(1, 10);
    

    assert!( arr2.contains(&10));
    assert_eq!(arr2.end(), &10);

}
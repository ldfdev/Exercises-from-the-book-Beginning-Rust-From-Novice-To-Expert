
fn main() {
    let _var = if true {123.001} else {001.123};
    // conditional expression must evaluate to te same type if all if/else branches
    // conditional expression evaluates as the last statement not terminated by ;

    let _simple_x =
        if true {
            let mut y = 0;
            y += 1;
            y *= 3;
            y += y - 1;
            1000000 + y
        }
        else {
            0
        };
}
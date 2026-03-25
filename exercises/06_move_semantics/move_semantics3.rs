// TODO: Fix the compiler error in the function without adding any new line.
//
// i was confused in that isn't vec0 in the test function below immutable(?)
//
// claude said this trips many people and said the key is that `mut` describes
// the variable binding, not the data itself. the analogy is like handing a box
// to someone. whether we labeled it as "do not modify", it doesn't matter once
// we've handed the box away, the new owner can do whatever they want with it,
// including relabeling it as mutable.
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    //          ^^^ this mut applies to the NEW binding inside this function
    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}

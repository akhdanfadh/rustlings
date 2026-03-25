// my first approach here (in comment) compiled and technically correct but gives warning:
//   writing `&Vec` instead of `&[_]` involves a new object where a slice will do
//
// the final solution just use vec.clone() directly in the test without modifying
// the intent of the initial function which is "give me a vector i can own and modify".
// the first approach here can be a bit misleading intent in that it basically says
// "give me a read-only reference" but then secretly clones and modifies it inside
//
// back to the warning, when we only need to read a vector, rust prefers to accepts
// &[i32] (a slice) instead of &Vec<i32>, because:
// - &[i32] is more flexible; it accepts Vec, arrays, or any contiguous sequence
// - &Vec<i32> forces the caller to have a Vec specifically
//
// ```
// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.clone();
// .
// .
//        let vec1 = fill_vec(&vec0);
// .
// ```

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}

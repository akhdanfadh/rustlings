fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }
        // after this, it will be:
        // [None, Some(1), Some(2), ..., Some(10)]

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        //
        // so the popped result type is Option<Option<i8>>
        // minimal flow example:
        // - 1st iter: vec is [None, Some(1)]
        //  pop() removes Some(1) -> returns Some(Some(1))
        //  matches Some(Some(i)) -> i = 1
        // - 2nd iter: vec is [None]
        //  pop() removes None -> returns Some(None)
        //  does not match Some(Some(i))? -> loop exits
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}

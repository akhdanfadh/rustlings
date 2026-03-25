fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);
        // compiler see that at this point, y variable won't be used anymore, so okay
        // another crucial key not or i missed in the book:
        // a reference's active scope ends at its LAST USE, not at the end of enclosing block
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}

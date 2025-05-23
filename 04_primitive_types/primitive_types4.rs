fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        
         // Note that the upper index 4 is excluded.
         let nice_slice = &a[1..4];
         assert_eq!([2, 3, 4], nice_slice);
 
         // The upper index can be included by using the syntax `..=` (with `=` sign)
         let nice_slice = &a[1..=3];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
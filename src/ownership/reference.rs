pub fn reference_1() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_1() {
        // same result
        let i = &99;
        let ref i = 99;

        let ref mut j = 99;
        *j += 1;// doesn't have ownership, but it can change the value.
        assert_eq!(j, &100);
    }
}

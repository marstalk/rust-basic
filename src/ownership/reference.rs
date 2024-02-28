pub fn reference_1() {}

#[cfg(test)]
mod tests {

    #[test]
    fn test_reference_1() {
        // same result
        let _i = &99;
        let ref _i = 99;

        let ref mut j = 99;
        *j += 1; // doesn't have ownership, but it can change the value.
        assert_eq!(j, &100);
    }
}

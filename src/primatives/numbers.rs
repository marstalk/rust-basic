/**
 * check operation can avoid overflow
 */
pub fn add_one_checked(i: i32) -> i32{
    match i.checked_add(1) {
        Some(res) => res,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_operation() {
        assert_eq!(add_one_checked(3), 4);
        assert_eq!(add_one_checked(i32::MAX), 0);
    }
}
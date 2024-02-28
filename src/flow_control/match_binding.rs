pub fn age() -> u16 {
    80
}

pub fn match_age() -> String {
    match age() {
        x if x >= 77 => format!("age is greater than 77 {}", x),
        _ => String::from("other"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_age() {
        assert_eq!(match_age(), "age is greater than 77 80");
    }
}

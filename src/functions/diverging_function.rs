///https://doc.rust-lang.org/rust-by-example/fn/diverging.html
pub fn odd_num_sum(n: u32) -> u32 {
    let mut sum = 0;
    for i in 0..n {
        // notice that the match expression return return type of u32.
        let odd = match i % 2 {
            1 => i,
            // but here return continue
            // TODO and this is diverging function here.
            _ => continue,
        };
        sum += odd;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_digerging_function() {
        assert_eq!(odd_num_sum(10), 25);
    }
}

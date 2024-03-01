/***
 * Find the sum of all the numbers with odd squares under 1000
 */

/// functional style
pub fn sum_of_odd_under(n: i32) -> i64 {
    (0..)
        .map(|x| x * x)
        // take_while is return false, then the stream is stop.
        .take_while(|&x_square| x_square < n)
        .filter(|&x_square| x_square % 2 == 1)
        .map(|x| x as i64)
        .sum()
}

/// imperative style
pub fn sum_of_odd_number_imperative(n: i32) -> i64 {
    let mut sum = 0;
    for x in 0.. {
        let x_square = x * x;
        if x_square >= n {
            break;
        }
        if x_square % 2 == 1 {
            sum += x_square as i64;
        }
    }
    sum
}

/// find all even number under N(inclusive)
pub fn all_even_number_under(n: i32) -> Vec<i32> {
    (0..=n).filter(|&x| x % 2 == 0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_even_number_under() {
        assert_eq!(all_even_number_under(10), vec![0, 2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_sum_of_odd_under_imperative() {
        assert_eq!(sum_of_odd_number_imperative(1000), 5456);
    }

    #[test]
    fn test_sum_of_odd_under() {
        assert_eq!(sum_of_odd_under(1000), 5456);
    }
}

use std::cmp::Ordering;

pub fn sort_vec(v: &mut Vec<i32>) {
    v.sort();
}

pub fn sort_vec2<F>(v: &mut Vec<i32>, f: F)
where
    F: Fn(&i32, &i32) -> Ordering,
{
    v.sort_by(f);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_vec() {
        let mut v = vec![3, 1, 4, 2];
        sort_vec(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_sort_vec2() {
        let mut v = vec![3, 1, 4, 2];
        sort_vec2(&mut v, |a, b| a.cmp(b));
        assert_eq!(v, vec![1, 2, 3, 4]);

        sort_vec2(&mut v, |a, b| b.cmp(a));
        assert_eq!(v, vec![4, 3, 2, 1]);
    }
}

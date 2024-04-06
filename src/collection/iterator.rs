/**
 * iterator.chain(iterator) link two iterator, return a new iterator.
 */
pub fn chain(v1: Vec<i32>, v2: Vec<i32>) -> String {
    let a = v1.iter().chain(v2.iter());
    a.map(|x| x.to_string()).collect()
}

/**
 * x.iterator.zip(y.iterator) return iterator((x, y)), use tuple to iterator.
 * either one is None, then return None and stop.
 */
pub fn zip(v1: Vec<i32>, v2: Vec<i32>) -> String {
    let a = v1.iter().zip(v2);
    a.map(|(x, y)| {
        let mut s = x.to_string();
        s.push_str(&y.to_string());
        s
    })
    .collect()
}

pub fn enumerate(v: Vec<i32>) -> String {
    let mut res = String::from("");
    for (i, x) in v.iter().enumerate() {
        res.push_str(&format!("{}:{},", i, x));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerate() {
        let v = vec![1, 2, 3];
        let s = enumerate(v);
        assert_eq!(s, "0:1,1:2,2:3,");
    }

    #[test]
    fn test_zip() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5, 6, 7];
        let s = zip(v1, v2);
        assert_eq!(s, "142536");
    }

    #[test]
    fn test_chain() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5, 6, 7, 8];
        let s = chain(v1, v2);
        assert_eq!(s, "12345678");
    }
}

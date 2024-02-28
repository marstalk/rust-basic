use std::collections::HashMap;

pub fn insert(map: &mut HashMap<String, i32>, key: String, value: i32) {
    map.insert(key, value);
}

/// create hashmap from array.
pub fn from_array(arr: [(String, i32); 2]) -> HashMap<String, i32> {
    HashMap::from(arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_array() {
        let map = from_array([(String::from("chengdu"), 30), (String::from("beijing"), 10)]);
        assert_eq!(map.get("chengdu").unwrap(), &30);
        assert_eq!(map.get("beijing").unwrap(), &10);
        assert_eq!(map.get("shanghai"), None);
    }

    #[test]
    fn test_insert() {
        let mut map: HashMap<String, i32> = HashMap::new();
        let k = String::from("hello");
        insert(&mut map, k, 3);
        // println!("k is moved, which is invalid. {}", k);

        assert_eq!(map.get("hello"), Some(&3));
        assert_eq!(map.get("hello").unwrap(), &3);
    }
}

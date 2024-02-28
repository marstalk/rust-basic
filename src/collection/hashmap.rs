use std::collections::HashMap;


pub fn insert(map: &mut HashMap<String, i32>, key: String, value: i32){
    map.insert(key, value);
}




#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_insert(){
        let mut map: HashMap<String, i32> = HashMap::new();
        insert(&mut map, String::from("hello"), 3);

        assert_eq!(map.get("hello"), Some(&3));
        assert_eq!(map.get("hello").unwrap(), &3);
    }
}
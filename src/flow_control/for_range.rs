/***
 * 1. for range
 * 2. vector/array/tuple iter(), iter_mut(), into_iter()
 * 3.
 */

use std::collections::HashMap;

pub fn consume_vec(mut v: Vec<i32>) {
    for item in v.iter() {
        println!("item: {}", item);
    }

    // mutable iteration
    for item in v.iter_mut() {
        *item += 1
    }
}

pub fn consume_vec_ref(v: &Vec<i32>) {
    // for item in v.iter_mut(){
    //     println!("item: {}", item);
    // }

    for item in v.into_iter() {
        println!("item: {}", item);
    }
    for item in v.into_iter() {
        println!("item: {}", item);
    }
    println!("{:?}", v);
}

pub fn consume_vec3(v: Vec<i32>) {
    for item in v.into_iter() {
        println!("item: {}", item);
    }
    //println!("{:?}", v);
}

pub fn consume_array(arr: [i32; 2]) -> String {
    let mut rtn = String::from("");
    for item in arr.iter() {
        rtn.push_str(&item.to_string());
    }
    rtn
}

pub fn add_two_for_each(arr: &mut [i32; 3]) {
    for item in arr.iter_mut() {
        *item += 1;
    }

    // equals to into_iter(), so arr is no longer valid.
    for item in arr {
        *item += 1;
    }
    // println!("{:?}", arr);
}

// doesn't support tuple
// pub fn consume_tuple(tu: &mut (i32, String)){
//     for (idx, item) in tu.iter(){

//     }
// }

pub fn consume_hashmap(map: &mut HashMap<String, i32>) {
    for (k, v) in map.iter_mut() {
        println!("{} updated value", k);
        *v += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_hashmap() {
        let mut map = HashMap::from([(String::from("apple"), 1), (String::from("banana"), 3)]);
        consume_hashmap(&mut map);
        assert_eq!(map.get("apple"), Some(&2));
        assert_eq!(map.get("banana"), Some(&4));
    }

    #[test]
    fn test_add_one_for_each() {
        let mut arr = [1, 2, 3];
        add_two_for_each(&mut arr);
        assert_eq!(arr, [3, 4, 5]);
    }

    #[test]
    fn test_consume_array() {
        assert_eq!(consume_array([1, 2]), "12");
    }

    #[test]
    fn test_into_iter() {
        consume_vec_ref(&vec![1, 2, 3, 4, 5]);
    }
}

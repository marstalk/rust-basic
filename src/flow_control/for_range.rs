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

pub fn add_one_for_each(arr: &mut [i32; 3]) {
    for item in arr {
        *item += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one_for_each() {
        let mut arr = [1, 2, 3];
        add_one_for_each(&mut arr);
        assert_eq!(arr, [2, 3, 4]);
    }

    #[test]
    fn test_consume_array() {
        assert_eq!(consume_array([1, 2]), "12");
    }

    #[test]
    fn test_into_iter() {
        consume_vec_ref(&vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_iter() {
        let vec = vec![1, 2, 3, 4, 5];
        for item in vec.iter() {
            println!("item: {}", item);
        }
        // vec.iter() simple borrow rather than move ownership
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);

        // compile fail
        // for item in vec.iter(){
        //     item += 1;
        // }
    }

    #[test]
    fn test_iter_mut() {
        let mut vec = vec![1, 2, 3, 4, 5];
        for item in vec.iter_mut() {
            *item += 1;
        }
        assert_eq!(vec, vec![2, 3, 4, 5, 6]);
    }
}

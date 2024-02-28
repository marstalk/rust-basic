pub fn consume_vec(mut v: Vec<i32>) {
    for item in v.iter() {
        println!("item: {}", item);
    }

    // can't mutable iteration
    for item in v.iter_mut() {
        *item += 1
    }
}

pub fn consume_vec_ref(mut v: &Vec<i32>) {
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

#[cfg(test)]
mod tests {
    use super::*;

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

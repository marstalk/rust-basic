#[allow(dead_code)]

pub struct User {
    name: String,
    age: i32,
    addr: String,
    friend: Option<Box<User>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_moved() {
        let user = User {
            name: "zhangsan".to_string(),
            age: 18,
            addr: String::from("chengdu"),
            friend: None,
        };

        // age is i32(primative, stack value) is copy, not moved.
        let age = user.age;
        let age2 = user.age;

        // name is String(heap value) is moved, and student is partial moved.
        let name = user.name;
        //let name2 = student.name;

        let addr = user.addr;

        // since student is partial moved, student can't be used.
        // but the rest attributes can be used.(the unmoved attributes and primative type attributes)
        //let user = student;

        assert_eq!(age, 18);
        assert_eq!(age2, 18);
        assert_eq!(name, "zhangsan");
        assert_eq!(addr, "chengdu");
    }

    #[test]
    fn test_partial_borrow() {
        let mut user = User {
            name: "zhangsan".to_string(),
            age: 18,
            addr: String::from("chengdu"),
            friend: None,
        };
        let user_ref = &mut user;

        // user_ref.name is moved operation, but the user_ref is a borrow, so compile failed.
        // let name = user_ref.name;

        let name = &user_ref.name;
        assert_eq!(name, "zhangsan");

        let name2 = &user_ref.name;
        assert_eq!(name2, "zhangsan");

        let addr = &mut user_ref.addr;
        addr.push_str("shi");
        assert_eq!(addr, "chengdushi");

        addr.clear();
        assert_eq!(addr, "");

        let mut age = user_ref.age;
        age += 1;
        // still 19 of user's age.
        assert_eq!(user_ref.age, 18);
        assert_eq!(age, 19);

        let age = &mut user_ref.age;
        *age += 1;
        assert_eq!(*age, 19);
        assert_eq!(user_ref.age, 19);
    }

    #[test]
    fn test_nested_mutable() {
        let friend = User {
            name: "lisi".to_string(),
            age: 20,
            addr: String::from("beijing"),
            friend: None,
        };
        let mut user = User {
            name: "zhangsan".to_string(),
            age: 18,
            addr: String::from("chengdu"),
            friend: Some(Box::new(friend)),
        };

        let user_ref = &mut user;

        let friend_option = &mut user_ref.friend;
        let friend = friend_option.take();
        // 1. take() move the value from the option which is a mutable reference.
        assert_eq!(user_ref.friend.is_none(), true);

        // 2. move the value to the user_ref.friend.
        user_ref.friend = friend;
        assert_eq!(user_ref.friend.is_some(), true);
        // with the help of option.take(), we can move the value from a mutable reference.

        let _friend_option = &user_ref.friend;
        // 3. only the mutable reference or ownership can use the take() method.
        // let friend = _friend_option.take();
    }
}

#[derive(Debug)]
pub struct User{
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub male: bool, 
    pub score: f64,
    pub friends: Vec<String>,
    pub addr: Addr,
}

#[derive(Debug)]
pub struct Addr{
    pub province: String,
    pub city: String,
}

// ownership moved.
pub fn match_struct(user: User) -> String{
    let rtn = match user {
        User{age: 18, name, ..} => format!("{} is 18 years old", name),
        //User{score: 60f64, id, ..} => format!("{} is 60 score", id),
        User{name, addr: Addr{city,..}, age: 20,..} => format!("{} is from {}", name, city),
        _ => "other".to_string(),
    };
    // user is moved, so user is invalid
    // println!("user is {:?}", user);
    println!("rtn is {}", rtn);
    rtn
}

// borrow, immutable.
pub fn match_struct_ref(user: &User) -> String{
    let rtn = match user {
        User{age: 18, name, ..} => format!("{} is 18 years old", name),
        _ => "other".to_string(),
    };
    // user is still valid
    println!("name of user is {}", user.name);
    rtn
}

// borrow, mutable
pub fn match_struct_mut_ref_age_add_one(user: &mut User) -> String{
    let rtn = match user {
        User{age: 18, name, ..} => format!("{} is 18 years old", name),
        _ => "other".to_string(),
    };
    user.age += 1;
    // user is still valid.
    println!("name of user is {}", user.name);
    rtn
}

pub fn update_name(mut name: String) -> String{
    name.push_str("_name_updated");
    name
}

pub fn update_name_2(name: &mut String) {
    name.push_str("_name_updated");
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_update_name_2(){
        let mut tyl = User{
            id: 1,
            name: String::from("tangbanxian"),
            age: 18,
            male: true,
            score: 66.0f64,
            friends: vec![String::from("liuyouer"), String::from("laiage")],
            addr: Addr{
                province: String::from("Guangdong"),
                city: String::from("Shenzhen"),
            }
        };

        update_name_2(&mut tyl.name);
        assert_eq!(tyl.name, "tangbanxian_name_updated");
    }

    #[test]
    fn test_update_name(){
        let mut tyl = User{
            id: 1,
            name: String::from("tangbanxian"),
            age: 18,
            male: true,
            score: 66.0f64,
            friends: vec![String::from("liuyouer"), String::from("laiage")],
            addr: Addr{
                province: String::from("Guangdong"),
                city: String::from("Shenzhen"),
            }
        };

        let name = update_name(tyl.name.clone());
        tyl.name = name;
        assert_eq!(tyl.name, "tangbanxian_name_updated");

        // can't borrow partial moved value tyl
        // update_name(tyl.name);
        // println!("tyl is {:?}", tyl);
    }

    #[test]
    fn test_match_struct_mut_ref_age_add_one(){
        let mut tyl = User{
            id: 1,
            name: String::from("tangbanxian"),
            age: 18,
            male: true,
            score: 66.0f64,
            friends: vec![String::from("liuyouer"), String::from("laiage")],
            addr: Addr{
                province: String::from("Guangdong"),
                city: String::from("Shenzhen"),
            }
        };

        assert_eq!(match_struct_mut_ref_age_add_one(&mut tyl), "tangbanxian is 18 years old");
        assert_eq!(tyl.age, 19);

    }
    #[test]
    fn test_match_struct_ref(){
        let tyl = User{
            id: 1,
            name: String::from("tangbanxian"),
            age: 18,
            male: true,
            score: 66.0f64,
            friends: vec![String::from("liuyouer"), String::from("laiage")],
            addr: Addr{
                province: String::from("Guangdong"),
                city: String::from("Shenzhen"),
            }
        };
        assert_eq!(match_struct_ref(&tyl), "tangbanxian is 18 years old");
        // tyl is still valid.
        assert_eq!(tyl.name, "tangbanxian");
    }

    #[test]
    fn test_struct(){
        let tyl = User{
            id: 1,
            name: String::from("tangbanxian"),
            age: 18,
            male: true,
            score: 66.0f64,
            friends: vec![String::from("liuyouer"), String::from("laiage")],
            addr: Addr{
                province: String::from("Guangdong"),
                city: String::from("Shenzhen"),
            }
        };

        assert_eq!(match_struct(tyl), "tangbanxian is 18 years old");
        // tyl is moved, so tyl is invalid
        //assert!("tyl is {:?}", tyl);

        let laiage = User{
            id: 2,
            name: String::from("laiage"),
            age: 20,
            male: false,
            score: 10.002f64,
            friends: vec![String::from("liuyouer"), String::from("tanbanxian")],
            addr: Addr{
                province: String::from("Guangdong"),
                city: String::from("xiamen"),
            }
        };
        assert!(match_struct(laiage) == "laiage is from xiamen")

    }

}
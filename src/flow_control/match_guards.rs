use super::match_struct::{Addr, User};

/// guards make it possible to match with condition.

pub fn guards(i: i32) -> String {
    match i {
        x if x > 0 => "positive".to_owned(),
        x if x < 0 => "negative".to_owned(),
        _ => "zero".to_owned(),
    }
}

pub fn guards_user(user: &User) -> String {
    match user {
        User { age, name, .. } if *age >= 18 => format!("{} is adult", name),
        User {
            name,
            addr: Addr { city, .. },
            ..
        } if city == "xiamen" => format!("{} is from xiamen", name),
        _ => "teenager".to_owned(),
    }
}

pub fn destruct_guards(user: &User) -> String {
    match user {
        User {
            id: id_value @ 1..=5,
            ..
        } => {
            format!("id={} from [1,5]", id_value)
        }
        _ => String::from("other"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_destruct_user() {
        let mut tyl: User = User {
            id: 4,
            name: String::from("tangbanxian"),
            age: 18,
            male: true,
            score: 66.0f64,
            friends: vec![String::from("liuyouer"), String::from("laiage")],
            addr: Addr {
                province: String::from("Guangdong"),
                city: String::from("Shenzhen"),
            },
        };
        let res = destruct_guards(&tyl);
        assert_eq!("id=4 from [1,5]", res);

        tyl.id = 8;
        assert_eq!("other", destruct_guards(&tyl));
    }

    #[test]
    fn test_guards_user() {
        let tyl: User = User {
            id: 1,
            name: String::from("tangbanxian"),
            age: 18,
            male: true,
            score: 66.0f64,
            friends: vec![String::from("liuyouer"), String::from("laiage")],
            addr: Addr {
                province: String::from("Guangdong"),
                city: String::from("Shenzhen"),
            },
        };
        assert_eq!(guards_user(&tyl), "tangbanxian is adult");
        assert_eq!(tyl.name, "tangbanxian");

        let laiage: User = User {
            id: 2,
            name: String::from("laiage"),
            age: 15,
            male: false,
            score: 10.002f64,
            friends: vec![String::from("liuyouer"), String::from("tanbanxian")],
            addr: Addr {
                province: String::from("Guangdong"),
                city: String::from("xiamen"),
            },
        };
        assert_eq!(guards_user(&laiage), "laiage is from xiamen");
        assert_eq!(laiage.name, "laiage");
    }

    #[test]
    fn test_guard() {
        assert_eq!(guards(1), "positive");
        assert_eq!(guards(-1), "negative");
        assert_eq!(guards(0), "zero");
    }
}

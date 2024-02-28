pub fn get_string_by_number(n: i32) -> String {
    let rtn = match n {
        1 => "one".to_owned(),
        2 => "two".to_owned(),
        3 => "three".to_owned(),
        4..=10 => "four to ten".to_owned(),
        11 | 12 => "eleven or twelve".to_owned(),
        x if x > 20 && x < 30 => format!("20~30 : {}", x),
        _ => "other".to_owned(),
    };

    rtn
}

pub fn no_destruct_tuple(tu: (i32, i32, String, i32)) -> &'static str {
    let rtn = match tu {
        (1, ..) => "first is one, other does't matter",
        (.., 2) => "last is two",
        // when there is no variable to bind, there is not destruction.
        // (5, x, y, z) => &format!("five, other is: {}, {}, {}", x, y, z),
        _ => "other",
    };

    println!("{:?}", tu);
    rtn
}
pub fn destruct_tuple(tu: (i32, i32, String, i32)) -> String {
    let rtn = match tu {
        (1, ..) => "first is one, other does't matter".to_owned(),
        (.., 2) => "last is two".to_owned(),
        (5, x, y, z) => format!("five, other is: {}, {}, {}", x, y, z),
        _ => "other".to_owned(),
    };

    // when there is variable binding, then a tu will be destructed. and tu is invalid.
    // println!("{:?}", tu);
    rtn
}

pub fn match_array(arr: [&str; 3]) -> String {
    let rtn = match arr {
        ["a", ..] => String::from("first is a"),
        [.., "c"] => String::from("last is c"),
        _ => String::from("other"),
    };

    // there is no variable binding, so no destruction happen, arr is still valid.
    println!("array is {:?}", arr);
    rtn
}

pub fn destruct_array(arr: [&str; 3]) -> String {
    let rtn = match arr {
        ["a", ..] => "first is a".to_owned(),
        [.., "c"] => "last is c".to_owned(),
        [a, b, "gg"] => format!("{} {} gg", a, b),
        _ => "other".to_owned(),
    };
    //TODO why arr is staill valid after destruction????
    println!("array is {:?}", arr);
    rtn
}

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue(i32, i32, i32),
}

pub fn match_enum(color: Color) -> String {
    let rtn = match color {
        Color::Red => "red".to_owned(),
        Color::Green => "green".to_owned(),
        Color::Blue(a, b, c) => format!("blue {} {} {}", a, b, c),
    };
    //TODO why color is still valid after destruction???
    println!("color is {:?}", color);
    rtn
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_color() {
        assert_eq!(match_enum(Color::Red), "red");
        assert_eq!(match_enum(Color::Green), "green");
        assert_eq!(match_enum(Color::Blue(1, 2, 3)), "blue 1 2 3");
    }

    #[test]
    fn test_destruct_array() {
        assert_eq!(destruct_array(["a", "b", "c"]), "first is a");
        assert_eq!(destruct_array(["c", "c", "c"]), "last is c");
        assert_eq!(destruct_array(["hello", "rust", "gg"]), "hello rust gg");
        assert_eq!(destruct_array(["ee", "ee", "ee"]), "other");
    }

    #[test]
    fn test_match_array() {
        assert_eq!(match_array(["a", "ee", "ee"]), "first is a");
        assert_eq!(match_array(["b", "c", "c"]), "last is c");
        assert_eq!(match_array(["d", "e", "e"]), "other");
    }

    #[test]
    fn test_destruct_tuple() {
        assert_eq!(
            destruct_tuple((5, 2, String::from("hello"), 4)),
            "five, other is: 2, hello, 4"
        );
    }

    #[test]
    fn test_no_destruct_tuple() {
        assert_eq!(
            no_destruct_tuple((1, 2, String::from("hello"), 3)),
            "first is one, other does't matter"
        );
        assert_eq!(
            no_destruct_tuple((2, 3, String::from("hello"), 2)),
            "last is two"
        );
        assert_eq!(no_destruct_tuple((9, 9, String::from("hello"), 9)), "other");
    }

    #[test]
    fn test_match_expr() {
        assert_eq!(get_string_by_number(1), "one");
        assert_eq!(get_string_by_number(2), "two");
        assert_eq!(get_string_by_number(3), "three");
        assert_eq!(get_string_by_number(4), "four to ten");
        assert!(get_string_by_number(11) == "eleven or twelve");
        assert_eq!(get_string_by_number(30), "other");
        assert_eq!(get_string_by_number(22), "20~30 : 22");
        assert_eq!(get_string_by_number(24), "20~30 : 24");
    }
}

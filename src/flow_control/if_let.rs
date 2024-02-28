use super::match_expr::Color;

/// if let could make [match] simple to avoid handle the None case.

pub fn if_let(str: Option<String>) -> Option<String>{
    if let Some(mut str) = str {
        str.push_str(" is a some");
        return Some(str);
    }
    // str is still valid.
    println!("str is invalid {:?}", str);
    None
}

pub fn if_let2(str: &mut Option<String>){
    if let Some(str) = str{
        str.push_str(" is a some");
    }
    // str is still valid.
    println!("str is valid {:?}", str);
}

pub fn if_let_enum(color: Color) -> String{
    if let Color::Blue(x, y, z) = color {
        return format!("blue is {}, {}, {}", x, y, z);
    }

    "other color".to_owned()
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_if_let_enum(){
        assert_eq!(if_let_enum(Color::Blue(1,2,3)), "blue is 1, 2, 3");
        assert_eq!(if_let_enum(Color::Red), "other color");
    }

    #[test]
    fn test_if_let2(){
        let mut name_option = Some(String::from("welcome"));
        if_let2(&mut name_option);
        assert_eq!(name_option, Some("welcome is a some".to_owned()));
    }

    #[test]
    fn test_if_let(){
        let some = Some("hello".to_string());
        assert_eq!(if_let(some), Some(String::from("hello is a some")));
        // Some is moved.
        // assert_eq!(some.take(), Some("hello".to_owned()));
        assert_eq!(if_let(None), None);
    }
}
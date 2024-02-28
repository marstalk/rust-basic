// to match reference, use & in branches or match *i
pub fn match_reference_i32(i: &i32) -> String {
    let rtn = match i {
        &1 => "one".to_owned(),
        &2 => "two".to_string(),
        // destrcturing
        &val => format!("other is {}", val),
        // after destructuring, the _ is unreachable.
        //_ => "other".to_string(),
    };
    println!("{}", i);
    rtn
}

// 2.0 use * to dereference.
pub fn match_reference_i64(i: &i64) -> String {
    match *i {
        1 => "one".to_owned(),
        2 => "two".to_string(),
        _ => "other".to_string(),
    }
}

// 3.0 "hello" is &str, so there is no need to *
pub fn match_reference_str(s: &str) -> String {
    match s {
        "hello" => "hello".to_owned(),
        "rust" => "rust".to_owned(),
        _ => "other".to_owned(),
    }
}

// 4.0 String match
pub fn match_string(s: String) -> String {
    match s {
        wothehell => wothehell.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_string() {
        assert_eq!(match_string("hello".to_owned()), "hello".to_owned());
    }

    #[test]
    fn test_match_reference_i32() {
        assert_eq!(match_reference_i32(&1), "one");
        assert_eq!(match_reference_i32(&2), "two");
        assert_eq!(match_reference_i32(&3), "other is 3");
        assert_eq!(match_reference_i32(&4), "other is 4");
    }
}

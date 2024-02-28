use std::str::FromStr;

/// given a string like '3 chair', return tuple (3, chair), if parse faile return (0, item);
pub fn get_count_item(str: &str) -> (i32, String){
    let mut str = str.split(" ");
    let default = (0, "item".to_owned());
    let (Some(count), Some(item)) = (str.next(), str.next()) else{
        return default;
    };
    let Ok(count) = i32::from_str(count) else{
        return default;
    };

    (count, String::from(item))
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_get_count_item(){
        assert_eq!(get_count_item("4 chair"), (4, "chair".to_owned()));
        assert_eq!(get_count_item("55people"), (0, "item".to_owned()));
    }
}


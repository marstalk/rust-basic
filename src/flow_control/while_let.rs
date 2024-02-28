
// given a option<i32>, 
// if none, return 0
// if some greater than or equal 9, return directly
// else increase by 1, until it's 9.
pub fn loop_and_match(mut i_option: Option<i32>) -> i32{
    loop{
        match i_option {
            Some(i) => {
                if i >= 9 {
                    break i;
                }else{
                    i_option = Some(i + 1);
                }
            },
            None => {
                break 0;
            }
        }
    }
}


/// while_let version to optimize loop_and_match.
pub fn while_let(mut i_option: Option<i32>) -> i32{
    while let Some(i) = i_option{
        if i >= 9 {
            return i;
        }
        i_option = Some(i + 1);
    }

    0
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test_loop_and_match(){
        assert_eq!(loop_and_match(Some(5)), 9);
        assert_eq!(loop_and_match(Some(10)), 10);
        assert_eq!(loop_and_match(None), 0);
    }
}
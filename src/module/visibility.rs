// sub module can call all the method of parent.
// parent

use self::sub::sub_hello_public;

fn parent_hello_private() {
    println!("Hello World!");
}

pub fn parent_hello_public() {
    println!("Hello World!");
}

pub fn some() {
    // can call sub public
    sub_hello_public();
    // can't call sub private
    //sub_hello_private()
}

mod sub {
    use super::*;

    pub fn sub_hello_public() {
        parent_hello_private();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        sub::sub_hello_public();
        assert!(true);
    }

    #[test]
    fn test_call_parent_function() {
        parent_hello_private();
        assert!(true);
    }
}

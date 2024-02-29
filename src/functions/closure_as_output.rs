// return must be : impl Fn() or impl FnMut() or impl FnOnce()
// return must be : move |xx|{yy}
pub fn create_fn() -> impl Fn() {
    let a = String::from("hello");
    // since return is Fn which is immutable borrow, so here it's invalid for mutable borrow.
    //move || a.push_str("_fn")
    move || println!("{}", a)
}

// supplier
pub fn create_fn_return() -> impl Fn() -> String {
    move || String::from("hello rust")
}

// function TODO

// consumer TODO
// pub fn create_fn_take_argument() -> impl Fn(){
//     move |x:i32| x+1
// }

#[cfg(test)]
mod tests {
    use super::{create_fn, create_fn_return};

    #[test]
    fn test_fn_return() {
        assert_eq!(create_fn_return()(), "hello rust");
    }

    #[test]
    fn test_fn() {
        create_fn()();
    }
}

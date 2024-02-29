fn create_fn() -> impl Fn() {
    let a = String::from("hello");
    // since return is Fn which is immutable borrow, so here it's invalid for mutable borrow.
    //move || a.push_str("_fn")
    move || println!("{}", a)
}

#[cfg(test)]
mod tests {
    use super::create_fn;

    #[test]
    fn test_fn() {
        create_fn()();
    }
}

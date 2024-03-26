use std::cell::RefCell;

/**
 * by default, the borrowing rules is applied in the compiling time:
 * 1. At any given time, you can have either (but not both) one mutable reference or any number of immutable references.
 * 2. References must always be valid.
 *
 * but with the help of RefCell, we can enforce the borrowing rules in the runtime.
 *
 * For those reasons, checking the borrowing rules at compile time is the best choice in the majority of cases,
 * which is why this is Rust’s default.
 *
 * Halting Problem
 */

pub struct User(i32);

pub fn immutable_borrow(user: &User) {
    println!("{}", user.0);
}

pub fn immutable_borrow2(user: RefCell<User>) -> User {
    user.borrow_mut().0 = 2;
    User(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refcell() {
        let user = User(1);
        immutable_borrow(&user);
        assert_eq!(user.0, 1);

        immutable_borrow2(RefCell::new(user));
    }
}

/***
*  In order of decreasing restriction, they are:
   Fn: the closure uses the captured value by reference (&T)
   FnMut: the closure uses the captured value by mutable reference (&mut T)
   FnOnce: the closure uses the captured value by value (T)
*/

use std::fmt::Display;

pub fn apply_fn_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

pub fn apply_fn_mut<F>(mut f: F)
where
    F: FnMut(),
{
    f();
}

pub fn apply_fn<F>(f: F)
where
    F: Fn(),
{
    f();
}

pub fn apply_fn2<F: Fn()>(f: F) {
    f();
}

pub fn apply_fn3<F: FnOnce() -> String>(f: F) -> String {
    let mut s = f();
    s.push_str(" updated");
    s
}

// consumer
pub fn consumer<F, T>(f: F, item: T)
where
    F: Fn(T),
{
    f(item)
}

// bifunction
pub fn operation<F, T>(f: F, x: T, y: T) -> T
where
    F: Fn(T, T) -> T,
{
    f(x, y)
}

// supplier
pub fn supplier<F: Fn() -> R, R: Display>(f: F) -> String {
    format!("{} is formatted", f())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        let addition = |x: i32, y: i32| x + y;
        assert_eq!(operation(addition, 1, 1), 2);

        let substract = |x: f64, y: f64| x - y;
        assert_eq!(operation(substract, 2.0f64, 1.1f64), 2.0 - 1.1);

        let concatenate = |mut x: String, y: String| {
            x.push_str(&y);
            x
        };
        assert_eq!(
            operation(concatenate, "hello".to_owned(), "rust".to_owned()),
            "hellorust"
        );
    }

    #[test]
    fn test_supplier() {
        let closure = || 5;
        assert_eq!(supplier(closure), "5 is formatted");
    }

    #[test]
    fn test_fn3() {
        let mut s = String::from("hello");
        let my_closure = || {
            s.push_str(" rust");
            s
        };

        let s = apply_fn3(my_closure);
        assert_eq!(s, "hello rust updated");
    }

    #[test]
    fn test_fn2() {
        let greeting = "hello rust";
        let farewell = String::from("good bye");
        let diary = || {
            // require immutable borrow only.
            assert_eq!(greeting, "hello rust");
            assert_eq!(farewell, "good bye");
        };

        apply_fn2(diary);
    }

    #[test]
    fn test_fn() {
        let greeting = "hello rust";
        let farewell = String::from("good bye");
        let diary = || {
            // require immutable borrow only.
            assert_eq!(greeting, "hello rust");
            assert_eq!(farewell, "good bye");
        };

        apply_fn(diary);
    }

    #[test]
    fn test_fn_mut() {
        let greeting = "hello rust";
        let mut farewell = String::from("good bye");

        let diary = || {
            // require immutable borrow.
            assert_eq!(greeting, "hello rust");

            // require mutable borrow.
            farewell.push_str("!!!");
        };
        // could use: mut fn
        apply_fn_mut(diary);
        assert_eq!(farewell, "good bye!!!");

        let diary = || {
            // require immutable borrow.
            assert_eq!(greeting, "hello rust");

            // require mutable borrow.
            farewell.push_str("!!!");
        };
        // could use fn once
        apply_fn_once(diary);
        assert_eq!(farewell, "good bye!!!!!!");
    }

    #[test]
    fn test_fn_once() {
        let greeting = "hello rust";
        let mut farewell = String::from("good_bye");

        let diary = || {
            // require immutable borrow.
            assert_eq!(greeting, "hello rust");

            // require mutable borrow.
            farewell.push_str("!!!");
            assert_eq!(farewell, "good_bye!!!");

            // require moved.
            std::mem::drop(farewell);
        };

        // compile failed.
        //apply2(diary);

        // diary require FnOnce.
        apply_fn_once(diary);
    }

    #[test]
    fn test_mutable_borrow() {
        let mut a = "hello rust".to_owned();
        let mutable_borrow_closre = || {
            a.push_str(" updated");
        };
        apply_fn_once(mutable_borrow_closre);

        // because closure of mutable_borrow_closure is still borrowing the a, so here can do immutable borrow.
        //assert_eq!(a, "hello rust updated");

        // since mutalbe_borrow_closure is moved. it's invalid.
        //apply(mutable_borrow_closre);
    }

    #[test]
    fn test_immutable_borrow_in_fnonce() {
        let a = String::from("hello rust");
        // 1. even though closure is annotated as FnOnce, but here require immutable borrow meet the needs
        // so here is immutable borrow.
        apply_fn_once(|| println!("{}", a));

        assert_eq!(a, "hello rust");
    }
}

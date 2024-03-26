use std::ops::Deref;

/***
 * MyBox1 VS myBox:
 * MyBox implement the Deref Trait providing the method deref() which can be used with *
 */
pub struct MyBox1<T>(T);

impl<T> MyBox1<T> {
    pub fn new(x: T) -> MyBox1<T> {
        MyBox1(x)
    }
}

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // remember always return reference.
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mybox() {
        let x = 5;
        let y = MyBox::new(x);
        // *y call the method of *(y.deref()) actually.
        assert!(5 == *y);
    }

    #[test]
    fn test_mybox1() {
        let x = 5;
        let _y = MyBox1::new(x);
        assert_eq!(5, x);

        // because the MyBox doesn't implement the Deref trait, we can't use the * operator to get the value out of it.
        // assert_eq!(5, *_y);
    }
}

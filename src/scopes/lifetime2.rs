#[derive(Debug)]
pub struct Foo {}

impl Foo {
    // 1.0 rule3, when input parameter is one only and it's &self mark as A,
    // the the return has the same lifetime as &self.
    pub fn mut_borrow(&mut self) -> &Self {
        //self
        // or
        &*self
    }

    pub fn borrow(&self) {}
}

// 4. 'a: 'b means the 'a lifetime must be valid at least as long as 'b: 'a >= 'b
pub struct Bar<'a: 'b, 'b, T> {
    pub r: &'a T,
    pub s: &'b T,
}

// 5. T:'a means T must be valid at least as long as 'a: T >='a
pub struct MyStruct<'a, T: 'a> {
    pub r: &'a T,
}

#[cfg(test)]
mod tests {
    use super::Foo;

    #[test]
    fn test_demo1() {
        let mut foo = Foo {};
        let loan = foo.mut_borrow();

        // 3. and here required the mutable borrow of self, which leads to conflicts.
        // so compile error
        // foo.borrow();

        // 2. and here the return is loan, so the loan require the the A is still valid.
        // which mean the self is still being mutable borrow.
        println!("{:?}", loan);
    }
}

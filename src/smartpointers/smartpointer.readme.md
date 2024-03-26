# Box VS Rc VS RefCell
1. Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
2. Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
3. Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

# RefCell
Interior Mutability: A Mutable Borrow to an Immutable Value


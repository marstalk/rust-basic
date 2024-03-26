use std::fmt::Display;

/**
 * demonstrate the Drop trait:
 * 1. custom the behavior when the object is dropped.
 */
struct Connection<T: Display>(i32, T);

impl<T: Display> Drop for Connection<T> {
    fn drop(&mut self) {
        println!(
            "drop Connection, release some resource here, {}, {}",
            self.0, self.1
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop() {
        {
            let _c = Connection::<String>(1, "hello".to_string());
            // when the _c out of scope then the _c.drop() method will be called.
        }
        println!("end")
    }
}

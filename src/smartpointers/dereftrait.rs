#[cfg(test)]
mod tests {

    #[test]
    fn test_int_refercen() {
        let x = 4;
        let y = x; // copy
        let z = &x; // reference
        println!("x: {}, y: {}, z: {}", x, y, z);

        //
        // if x == z {
        //     println!("x and z are equal");
        // }
        assert!(x == *z);
        // assert!(x == z);

        let x = 5;
        let y = Box::new(x);
        assert!(x == *y);
    }
}

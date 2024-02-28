#[cfg(test)]
mod tests {
    #[test]
    fn test_closure() {
        let mut x: i32 = 5;
        let my_operation = |i: i32| {
            // borrow x here.
            i + x
        };
        assert_eq!(my_operation(5), 10);

        // could'nt change x, becuase, x is still being borrowed by my_operation.
        //x = 10;
        //assert_eq!(my_operation(5), 15);

        // could change here, because x borrowed by my_operation is over.
        x = 10;
        assert_eq!(x, 10);
    }

    #[test]
    fn test_closure_immutable_borrow() {
        let mut x = 10;
        // immutable borrow
        let print_closure = || println!("{}", x);
        print_closure();

        // when doing a copy, it require a immutable borrow;
        let mut copied = x;
        print_closure();
        copied += 1;
        assert_eq!(copied, 11);
        assert_eq!(x, 10);

        x += 1;
        // after borrow is oever, x could be changed.
        assert_eq!(x, 11);
        // print_closure();
    }

    #[test]
    fn test_closure_mutable_borrow() {
        let mut x = 10;

        // must add [mut] here.
        let mut increase_and_print = || {
            x += 1;
            println!("x = {}", x);
        };

        increase_and_print();
        // can't copy x, because can't immutable borrow here while x is being mutable borrowed.
        // let y = x;
        increase_and_print();
    }

    #[test]
    fn test_copy() {
        let x = 10;
        let y = x;
        println!("{}, {}", x, y);
    }

    #[test]
    fn test_closure_moved() {
        let movable = Box::new(2);
        let print_and_free = || {
            println!("movable = {}", movable);
            std::mem::drop(movable);
        };

        print_and_free();

        // movable is invalid.
        // println!("{:?}", movable);
    }
}

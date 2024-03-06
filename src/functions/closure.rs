/**
* in clsoure. it will infer to use:
   1. immutable borrow.
   2. mutable borrow.
   3. move
 */

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
        // immutable borrow can meet the needs. so the immutable borrow happens here.
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
            // x is mutable here, so mutable borrow happen here.
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
            // drop require the ownership, so move happen here.
            std::mem::drop(movable);
        };

        print_and_free();

        // movable is invalid.
        // println!("{:?}", movable);
    }

    // use [move] keyword in front of vetical || indicate the closure to take ownership of captured.
    #[test]
    fn test_force_move() {
        let vec = vec![1, 2, 3];
        let moved_closure = move |item| vec.contains(item);
        assert_eq!(true, moved_closure(&1));
        assert_eq!(false, moved_closure(&4));

        // because vec is moved, means invalid.
        // println!("{:?}", vec);

        // once moved_closure is out of scope, then moved_closure and vec will be drop.
    }

    #[test]
    fn test_supplier() {
        let supplier = || String::from("hello");

        assert_eq!(supplier(), "hello");
    }

    #[test]
    fn test_function() {
        let function = |mut x: i32| {
            x += 1;
            format!("{} increased", x)
        };

        assert_eq!(function(1), "2 increased");
    }

    #[test]
    fn test_consumer() {
        let consumer = |x: String| {
            println!("{}", x);
        };

        consumer("hello".to_string());
    }

    #[test]
    fn test_bi_function() {
        let bi_function = |x: i32, y: String| -> String { format!("{}{}", x, y) };

        assert_eq!(bi_function(1, "hello".to_owned()), "1hello");
    }
}

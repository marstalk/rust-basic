#[derive(Debug)]
pub struct Student(i32);

impl Drop for Student {
    fn drop(&mut self) {
        println!("{:?} is dropped", self);
    }
}

pub fn simple_return(i: i32) -> i32 {
    i
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_value_ref() {
        let rc = Rc::new(4);
        assert_eq!(rc, Rc::new(4));
        // have to deref by hand
        println!("{:?}", simple_return(*rc));
    }

    #[test]
    fn test_student_rc() {
        {
            let _s = Student(1);
            std::mem::drop(_s);
            println!("_s is dropped");
        }

        let s1 = Rc::new(Student(2));
        let s2 = s1.clone();

        // won't call the drop method.
        std::mem::drop(s1);
        println!("{:?} s2 is still valid, won't deallocate", s2);

        // s2 is dropped, drop method will be called here.
        std::mem::drop(s2);
        println!("s2 is invalid, deallocated");
    }
}

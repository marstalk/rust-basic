#[derive(Debug)]
pub struct Student(i32);

impl Drop for Student {
    fn drop(&mut self) {
        println!("{:?} is dropped", self);
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;

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

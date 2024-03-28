use std::{
    sync::{Arc, Mutex},
    thread,
};

pub struct Student(i32);

pub fn custom_type_in_threads(student: Student) {
    let student = Arc::new(Mutex::new(student));

    let student_a = student.clone();
    let t = thread::spawn(move || {
        student_a.lock().unwrap().0 += 1;
    });

    let student_b = student.clone();
    let t2 = thread::spawn(move || {
        student_b.lock().unwrap().0 += 1;
    });

    t.join().unwrap();
    t2.join().unwrap();

    //TODO to be done.
}

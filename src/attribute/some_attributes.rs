// #![] used for the whole rs file.
#![allow(unused_variables)]
#![allow(dead_code)]
fn some_method() {
    let a = 1;
}

// #[] for the following item only
#[derive(Debug)]
struct MyStruct {
    a: i32,
    b: i32,
}

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cfg() {
        are_you_on_linux();

        println!("Are you sure?");
        if cfg!(target_os = "linux") {
            println!("Yes. It's definitely linux!");
        } else {
            println!("Yes. It's definitely *not* linux!");
        }
    }
}

/***
* lifetime的出现，只要是因为编译器不够聪明，有一些场景无法识别出某个变量是不是空指针。所以需要开发者通过一定的语法告诉编译器：
* 说变量A的生命周期小于等于变量B的生命周期。等等。而有了这个信息之后，编译器就会通过一下原则进行判断：
   - 生命周期大的变量无法引用生命周期较小的变量或者值。
   - 生命周期小的变量可以引用生命周期较大的变量或者值。
   如果编译器发现不合法的引用，就会报错。
*/

#![allow(unused_variables)]
#![allow(dead_code)]
use std::fmt::Display;

/**
 * use lifetime annotation to handle dangling reference.
 * together with
 * - generic type parameter to make function/method can handle different types.
 * - trait and trait bound to ensure logic
 * - lifetime to avoid dangling references.
 */

/**
* 1.
   - 生命周期大的变量无法引用生命周期较小的变量或者值。
   - 生命周期小的变量可以引用生命周期较大的变量或者值。
*/

pub fn borrow_limit() {
    // outer scope define r.
    let a = 2;
    let r = &a;
    {
        // declare x inner scope.
        let x = 5;
        // r of outer scopre make a reference to x
        //r = &x;
        // make sure you know the  diffence of below code.
        // let r = &x;
    } // x becomes invalid. so the r reference.
      // so the compiler found error here.
    println!("r: {}", r);
}

/**
 * 2. the method could return x or y in runtime.
 * so the borrow checker don't know the return &str valid scope.
 * we programmer should tell the checker.
 *
 * the returned reference will be valid as long as both the parameters are valid.
 *
 * It means that the lifetime of the reference returned by the longest function
 * is the same as the smaller of the lifetimes of the values referred to by the function arguments.
 * 主要是告诉borrow checker使用说明标准来检查声明周期，确保return str在被使用的时候，x或者y都还在。
 * 避免空指针。
 */
pub fn avoid_dangling_reference() {
    // this method could either return x or y's borrow depending on runtime.
    // in order to make return value not invalid, the lifetime of return must be <= min lifetime of (x, y)
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // lifetime annotation doesn't change the lifetime.
    // it's just describe the relationship of the lifetime of multiple references to each other.
    let str1 = String::from("hellow");
    let str2 = "Google";
    let str3 = longest(&str1, str2);
    println!("longest is {}", str3);
}

pub fn avoid_dangling_reference2() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    // str1 has a longer lifetime than str2
    // so the borrow checker will check
    // if the str3 lifetime is shorter or equal to str2(the shorter one).
    let str1 = String::from("Hello, world!");
    {
        let str2 = String::from("abc");
        let str3 = longest(&str1, &str2);
        println!("The longest string is {}", str3);
    } // str2 drop and invalid here. as the str3.
      // so str3 is invalid here, usage will occur compile failure.
      // str1 is still valid here.
    println!("str1 {}", str1);
}

pub fn no_lifetime_annotation() {
    // the checker know how to check. because the return value is always a reference of parameter.
    fn longest_v3(str: &str) -> &str {
        &str[..1]
    }
    //4. when borrow checker is smart enough to check, then we don'nt have to set lifetime.
    let str1 = String::from("hello");
    let str2 = longest_v3(&str1);

    // if str1.clear() is called, then the str2 is invalid. try to uncomment me.
    //str1.clear()
    println!("str2 {}", str2);
}

/***
 * 6. lifetime elision https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-struct-definitions

   The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
   In other words,
   a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
   a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

   The second rule is that, if there is exactly one input lifetime parameter,
   that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

   The third rule is that, if there are multiple input lifetime parameters,
   but one of them is &self or &mut self because this is a method,
   the lifetime of self is assigned to all output lifetime parameters.
   This third rule makes methods much nicer to read and write because fewer symbols are necessary.
*/
pub fn rule_of_elision_life_time() {}

pub fn static_life_time() {
    // 7. static lifetime.
    // this could affact the lifetime to live long as the program !!!
    let str1 = "I have a static lifetime.";
    let str2: &'static str = "I have a static lifetime.";
    // they are the same.
}

// 8. together with generic type parameter, trait bound and lifetime.
pub fn life_time_with_generic() {
    /**
     * complicated demo for generic types paramter, trait bounds and lifetime.
     */
    fn longest_with_an_announcement<'a, T>(
        // 在方法名之后使用<>把泛型和生命周期。
        x: &'a str, // x是str slice, mutable引用类型，且和return的lifetime建立了关系。
        y: &'a str, // 同上。
        ann: T,     // 泛型ann，move the ownershiop here. 不和return的lifetime建立关系。
    ) -> &'a str
    // return value is str slice. build lifetime relationship with x and y.
    // compiler checker will check return value lifetime is shorter than min(x lifetime, y lifetime)
    where
        T: Display, // trait bounds indicates that ann must have a trait of Display.
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let str1 = String::from("welcome to earth");
    let str2 = "welcome to mars";
    let str3 = String::from("Hello");
    let rtn = longest_with_an_announcement(&str1, str2, str3);
    println!("longest_with_an_annoucement is {}", rtn);
    //println!("str3 is invalid here, because ownership moved {}", str3);
    println!("str1 is still valid here {}", str1);
}

// if we always return the the x, then we don't have to use 'a for y
pub fn longest_v2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// fn longest_V4<'a> (x: &str, y: &str) -> &'a str{
//     let result = String::from("really long string");
//     result.as_str()
// }

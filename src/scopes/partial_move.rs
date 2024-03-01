/// partial moved VS partial reference(borrow)

pub struct Person {
    name: String,
    age: i32,
}

pub fn partial_moved(person: Person) -> String {
    // name and age are moved, so person is invalid
    let Person { name, ref age } = person;
    // compile fail
    //format!("{} is {} years old", person.name, person.age);
    format!("{} is {} years old", name, age)
}

pub fn partial_reference(person: Person) -> (Person, String) {
    let Person { ref name, ref age } = person;
    println!("{} is {} years old", name, age);
    (person, String::from("chengdu"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_reference() {
        let person: Person = Person {
            name: String::from("chengdu"),
            age: 30,
        };
        let (_, result) = partial_reference(person);
        assert_eq!(result, "chengdu");
    }

    #[test]
    fn test_partial_moved() {
        let person = Person {
            name: String::from("chengdu"),
            age: 30,
        };
        let result = partial_moved(person);
        assert_eq!(result, "chengdu is 30 years old");
    }
}

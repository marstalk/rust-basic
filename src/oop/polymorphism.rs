/**
 * use Trait to implement polymorphism
 *
 * 1. the components could be different type at runtime.
 */
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/**
 * compare with Generic and Trait Bound.
 * 2. the element in the vector must be the same type which implement the Draw trait .
 */
pub struct Screen2<T: Draw> {
    pub components: Vec<T>,
}
impl<T> Screen2<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/**
 * 3. implement Draw trait
 */
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!("draw button");
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw selectbox");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen() {
        //4. create a Screen instance
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 100,
                    height: 200,
                    options: vec![String::from("a"), String::from("b"), String::from("c")],
                }),
                Box::new(Button {
                    width: 100,
                    height: 200,
                    label: String::from("button"),
                }),
            ],
        };

        screen.run();
    }
}

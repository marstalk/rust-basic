pub struct Post {
    // 3. why Option? why Box? why dyn?
    // Option: for the mutable post, can use post.state.take() to move the box ownership.
    state: Option<Box<dyn State>>,
    content: String,
}
// 1. state pattern: the post struct rely on state strat only.
// if without state pattern, we have to use several branch arms which is overwhelm when more state is added to the system.
impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: "".to_string(),
        }
    }

    pub fn add_text(&mut self, text: String) {
        if self.state.as_ref().unwrap().can_add() {
            self.content.push_str(&text);
        }
    }

    pub fn request_review(&mut self) {
        if let Some(old_state) = self.state.take() {
            self.state = Some(old_state.request_review());
        }
    }

    pub fn approve(&mut self) {
        // ownership of state in Option<State> have moved and leave None behind only when the self is mutable
        if let Some(old_state) = self.state.take() {
            // state ownership move into method approve.
            self.state = Some(old_state.approve());
        }
    }

    /**
     * PendingReview -> Draft
     */
    pub fn reject(&mut self) {
        if let Some(old_state) = self.state.take() {
            self.state = Some(old_state.reject());
        }
    }

    pub fn content(&self) -> &str {
        // 2. use state to return content.
        // as_ref: reference to the value rather than take the ownership. &Option<V> --> Option<&V>
        //
        let option_state = self.state.as_ref();
        let a = option_state.unwrap();
        a.content(self)
    }
}

pub trait State {
    fn can_add(&self) -> bool;

    // 3.1 for those state transform method:
    // input is self rather than &self, &mut self
    // output is Box<dyn state> rather than &Box<dyn State> ****this is the most important**** -> input must be self -> use Option.take
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}
pub struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn can_add(&self) -> bool {
        true
    }
}

pub struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn can_add(&self) -> bool {
        false
    }
}

pub struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        // post is a reference. but the post.content try to get the ownership of content, which is impossible.
        // let a = post.content;

        // we can only get the reference of content only, but &post.content.
        // &post.content
        // or post.content.as_ref()
        post.content.as_ref()
    }

    fn can_add(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post() {
        let mut post = Post::new();
        post.add_text("hello".to_string());
        assert_eq!(post.content(), "");

        post.request_review();
        assert_eq!(post.content(), "");

        post.approve();
        assert_eq!(post.content(), "hello");
    }
}

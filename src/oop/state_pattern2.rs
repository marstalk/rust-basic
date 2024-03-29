/**
 * encoding the state into type system, to avoid some bugs by take advantages of Rust exclusive feature: ownership and shadowing
 */

pub struct Post {
    pub content: String,
}
impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(&text);
    }

    pub fn reqquest_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}
impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post() {
        let mut post = Post::new();

        post.add_text("hello");
        post.add_text(" text");

        let post = post.reqquest_review();
        let post = post.approve();
        assert_eq!(post.content, "hello text")
    }
}

//! # The Blog OOP Library
//!
//! Implements the "State" design pattern exactly as it is defined for object-oriented languages.
//!
//! - A review needs to be requested on a post that is in the "Draft" state before the post can be approved.
//! - Has a `reject` method that changes the post’s state from "PendingReview" back to "Draft".
//! - Requires two calls to `approve` before the state can be changed to "Published".
//! - Allows users to add text content only when a post is in the "Draft" state.


/// ## Post
/// A review needs to be requested before a post can be approved.
/// A post needs two approvals to become published.
/// A post can also be rejected. A rejection can happen before or after the first approval.
/// In either case, a rejection moves the post back into the "Draft" state, where it can be edited,
/// and then a new review request needs to be made.
/// A post can only be edited if it's in the "Draft" state.
/// A post's content can only be retrieved if the post has been published.
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    editable: bool,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Self::create_draft(),
            content: String::new(),
            editable: true,
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if self.editable {
            self.content.push_str(text);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
            self.editable = false;
        }
    }

    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
            self.editable = false;
        }
    }

    pub fn reject(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.reject());
            self.editable = true;
        }
    }

    fn create_draft() -> Option<Box<dyn State>> {
        Some(Box::new(Draft {}))
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

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
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(ApprovedOnce {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct ApprovedOnce {}

impl State for ApprovedOnce {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

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
        &post.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "Example OOP blog post content for unit-testing.";
    const IMPROVED: &str = " IMPROVED!";
    const IMPROVED_CONTENT: &str = "Example OOP blog post content for unit-testing. IMPROVED!";

    #[test]
    /// We can create a Post in a unit test in any possible state. Private fields are accessible.
    /// This is just for exercise. This is not a good practice.
    fn draft_post_returns_no_content() {
        let post = Post {
            state: Some(Box::new(Draft {})),
            content: CONTENT.to_string(),
            editable: true,
        };
        assert_eq!("", post.content());
    }

    #[test]
    /// We have to call `request_review()` from the "Draft" state, before `approve()`.
    fn draft_post_tried_to_approve_twice() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("", post.content());
    }

    #[test]
    fn draft_post_tried_to_reject() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        assert_eq!("", post.content());
        post.reject();
        assert_eq!("", post.content());
    }

    #[test]
    fn draft_post_to_pending_review() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        assert_eq!("", post.content());
        post.request_review();
        assert_eq!("", post.content());
    }

    #[test]
    /// We can create a Post in a unit test in any possible state. Private fields are accessible.
    /// This is just for exercise. This is not a good practice.
    fn post_under_review_returns_no_content() {
        let post = Post {
            state: Some(Box::new(PendingReview {})),
            content: CONTENT.to_string(),
            editable: true,
        };
        assert_eq!("", post.content());
    }

    #[test]
    /// Needs two approvals to be published.
    fn once_approved_post_returns_no_content() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        assert_eq!("", post.content());
        post.request_review();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("", post.content());
    }

    #[test]
    /// We have to call `request_review()` after each rejection, because it goes back to Draft.
    fn post_under_review_rejected_then_tried_to_approve_twice() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        post.request_review();
        post.reject();  // Moves it back to Draft.
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("", post.content());
    }

    #[test]
    fn once_approved_post_rejected_then_approved_twice() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        post.request_review();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("", post.content());
        post.reject();  // Moves it back to Draft.
        assert_eq!("", post.content());
        post.request_review();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!(CONTENT, post.content());
    }

    #[test]
    /// We have to call `request_review()` after each rejection, because it goes back to Draft.
    fn post_under_review_rejected_then_approved_twice() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        post.request_review();
        post.reject();  // Moves it back to Draft.
        assert_eq!("", post.content());
        post.request_review();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!(CONTENT, post.content());
    }

    #[test]
    fn published_post_returns_full_content() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        post.request_review();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!(CONTENT, post.content());
    }

    #[test]
    /// Test that we cannot reject an already published post.
    fn published_post_tried_to_reject() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        post.request_review();
        assert_eq!("", post.content());
        post.approve();
        post.approve();
        assert_eq!(CONTENT, post.content());
        post.reject();
        assert_eq!(CONTENT, post.content());
    }

    #[test]
    fn new_post_can_change_content() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        assert_eq!(CONTENT, post.content);
        post.add_text(IMPROVED);
        assert_eq!(IMPROVED_CONTENT, post.content);
    }

    #[test]
    fn post_under_review_cannot_change_content() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        assert_eq!(CONTENT, post.content);
        post.request_review();
        post.add_text(IMPROVED);
        assert_eq!(CONTENT, post.content);
    }

    #[test]
    fn once_approved_post_cannot_change_content() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        assert_eq!(CONTENT, post.content);
        post.request_review();
        post.approve();
        post.add_text(IMPROVED);
        assert_eq!(CONTENT, post.content);
    }

    #[test]
    fn rejected_post_can_change_content_1() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        assert_eq!(CONTENT, post.content);
        post.reject();
        post.add_text(IMPROVED);
        assert_eq!(IMPROVED_CONTENT, post.content);
    }

    #[test]
    fn rejected_post_can_change_content_2() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        assert_eq!(CONTENT, post.content);
        post.request_review();
        post.reject();
        post.add_text(IMPROVED);
        assert_eq!(IMPROVED_CONTENT, post.content);
    }

    #[test]
    fn rejected_post_can_change_content_3() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        assert_eq!(CONTENT, post.content);
        post.request_review();
        post.approve();
        post.reject();
        post.add_text(IMPROVED);
        assert_eq!(IMPROVED_CONTENT, post.content);
    }

    #[test]
    fn published_post_cannot_change_content() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        assert_eq!(CONTENT, post.content);
        post.request_review();
        post.approve();
        post.approve();
        post.add_text(IMPROVED);
        assert_eq!(CONTENT, post.content);
    }
}

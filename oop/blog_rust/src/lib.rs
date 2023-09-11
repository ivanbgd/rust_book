//! # The Blog Rust Library
//!
//! Implements the "State" design pattern using Rust's strengths (advantages), such as type-checking.
//!
//! States are encoded into different types.
//! The blog post workflow is encoded into the type system.
//!
//! - A review needs to be requested on a draft post before a post can be approved.
//! - Has a `reject` method that changes the post’s state from "PendingReview" back to "Draft".
//! - Requires two calls to `approve` before the state can be changed to "Published".
//! - Allows users to add text content only when a post is in the "Draft" state.


/// ## Draft Post
/// A post can only be edited if it's in the "Draft" state.
/// We can also ask for a review of a draft post.
pub struct DraftPost {
    content: String,
}

/// ## Pending Review Post
/// We can approve or reject a pending review post.
pub struct PendingReviewPost {
    content: String,
}

/// ## Approved Once Post
/// We can approve or reject a post that has been approved once.
pub struct ApprovedOncePost {
    content: String,
}

/// ## Published Post
/// Use this to create a new, empty, draft post.
/// A post's content can only be retrieved if the post has been published.
pub struct Post {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> ApprovedOncePost {
        ApprovedOncePost {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

impl ApprovedOncePost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "Example Rust blog post content for unit-testing.";
    const IMPROVED: &str = " IMPROVED!";
    const IMPROVED_CONTENT: &str = "Example Rust blog post content for unit-testing. IMPROVED!";

    #[test]
    /// Approve twice.
    fn create_review_approve_approve_show_content() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        let post = post.request_review();
        let post = post.approve();
        let post = post.approve();
        assert_eq!(CONTENT, post.content());
    }

    #[test]
    /// Reject, then approve twice.
    fn create_review_reject_review_approve_approve_show_content() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        let post = post.request_review();
        let mut post = post.reject();
        post.add_text(IMPROVED);
        let post = post.request_review();
        let post = post.approve();
        let post = post.approve();
        assert_eq!(IMPROVED_CONTENT, post.content());
    }

    #[test]
    /// Approve, reject, then approve twice.
    fn create_review_approve_reject_review_approve_approve_show_content() {
        let mut post = Post::new();
        post.add_text(CONTENT);
        let post = post.request_review();
        let post = post.approve();
        let mut post = post.reject();
        post.add_text(IMPROVED);
        let post = post.request_review();
        let post = post.approve();
        let post = post.approve();
        assert_eq!(IMPROVED_CONTENT, post.content());
    }
}

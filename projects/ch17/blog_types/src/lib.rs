pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
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

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approvals: 0,
        }
    }
}

pub enum PendingReviewResult {
    Pending(PendingReviewPost),
    Published(Post),
}

pub struct PendingReviewPost {
    content: String,
    approvals: i8,
}

impl PendingReviewPost {
    pub fn approve(self) -> PendingReviewResult {
        // pub fn approve(self) -> Either<PendingReviewPost, Post> {
        if self.approvals < 1 {
            PendingReviewResult::Pending(PendingReviewPost {
                content: self.content,
                approvals: self.approvals + 1,
            })
        } else {
            PendingReviewResult::Published(Post {
                content: self.content,
            })
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

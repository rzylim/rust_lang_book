use blog::{PendingReviewResult, Post};

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let mut post = post.approve();
    if let PendingReviewResult::Pending(pst) = post {
        post = pst.approve();
    };
    if let PendingReviewResult::Published(pst) = post {
        assert_eq!("I ate a salad for lunch today", pst.content());
    };
}

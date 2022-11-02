use post::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate samosas earlier today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate samosas earlier today", post.content());
}

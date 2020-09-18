#[derive(Debug)]
pub struct Post {
    pub title: String,
    pub url: String,
    pub num_comments: u64,
    pub up_votes: i64,
    pub permalink: String,
}

#[derive(Debug)]
pub struct PostView {
    pub self_text: Option<(String, i64)>,
    pub comments: Vec<Comment>,
}

#[derive(Debug)]
pub struct Comment {
    pub body: String,
    pub replies: Vec<Comment>,
    pub up_votes: i64,
}

#[derive(Debug)]
pub struct Post {
    pub title: String,
    pub url: String,
    pub num_comments: u64,
    pub up_votes: i64,
}

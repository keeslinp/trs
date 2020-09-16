use crate::model::Post;

#[derive(Debug)]
pub enum Msg {
    FetchSubreddit(String),
    SubredditResponse(Vec<Post>),
    Error(String),
    Down,
    Up,
    Quit,
}

use crate::model::Post;

#[derive(Debug)]
pub enum Msg {
    FetchSubreddit(Option<String>), // None fetches homescreen
    SubredditResponse(Vec<Post>),
    Error(String),
    Down,
    Up,
    Quit,
}

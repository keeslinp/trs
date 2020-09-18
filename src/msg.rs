use crate::model::{Post, PostView};

#[derive(Debug)]
pub enum Msg {
    FetchSubreddit(Option<String>), // None fetches homescreen
    SubredditResponse(Vec<Post>),
    Error(String),
    CommentsResponse(PostView),
    Prev,
    Next,
    Select,
    Quit,
    //TODO: Is this a good name?
    Up,   // Shallower
    Down, // Deeper
}

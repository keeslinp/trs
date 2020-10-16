use crate::model::{Post, PostView};
use termion::event::Key;

#[derive(Debug)]
pub enum Msg {
    FetchSubreddit(Option<String>), // None fetches homescreen
    SubredditResponse(Vec<Post>, Option<String>),
    Error(String),
    CommentsResponse(PostView),
    Input(Key),
}

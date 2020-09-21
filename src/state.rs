use crate::model::{Post, PostView};
use tui::widgets::ListState;

#[derive(Debug)]
pub enum State {
    SubList(Vec<Post>, ListState),
    PostView(PostView, ListState),
    Loading,
    SelectSubreddit(String),
}

impl Default for State {
    fn default() -> Self {
        State::Loading
    }
}

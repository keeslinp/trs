use crate::model::{Post, PostView};
use tui::widgets::ListState;

#[derive(Debug)]
pub enum View {
    SubList(Vec<Post>, ListState),
    PostView(PostView, ListState),
    Loading,
}

#[derive(Default)]
pub struct State {
    pub view_state: View,
}

impl Default for View {
    fn default() -> View {
        View::Loading
    }
}

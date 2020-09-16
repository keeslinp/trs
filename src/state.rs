use crate::model::Post;
use tui::widgets::ListState;

pub enum View {
    SubList(Vec<Post>, ListState),
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

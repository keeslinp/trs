use crate::model::Post;
pub enum View {
    SubList(Vec<Post>),
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

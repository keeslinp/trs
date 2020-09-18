use crate::{
    api::get_post_view,
    api::get_posts,
    msg::Msg,
    state::{State, View},
};
use anyhow::{bail, Result};
use futures::future::BoxFuture;
use tui::widgets::ListState;

pub fn update(msg: Msg, state: &mut State) -> Result<Option<BoxFuture<'static, Result<Msg>>>> {
    match msg {
        Msg::FetchSubreddit(sub) => {
            state.view_state = View::Loading;
            return Ok(Some(Box::pin(async move {
                let posts = get_posts(sub.as_deref()).await?;
                Ok(Msg::SubredditResponse(posts))
            })));
        }
        Msg::SubredditResponse(posts) => {
            let mut list_state = ListState::default();
            if !posts.is_empty() {
                list_state.select(Some(0));
            }
            state.view_state = View::SubList(posts, list_state);
        }
        Msg::Error(e) => {
            bail!(e);
        }
        Msg::Next => match &mut state.view_state {
            View::SubList(posts, ref mut list_state) => {
                list_state.select(list_state.selected().map(|s| {
                    if s < posts.len() - 1 {
                        s + 1
                    } else {
                        s
                    }
                }));
            }
            View::PostView(post_view, ref mut list_state) => {
                list_state.select(list_state.selected().map(|s| {
                    if s < post_view.comments.len() {
                        s + 1
                    } else {
                        s
                    }
                }));
            }
            View::Loading => {}
        },
        Msg::Prev => match &mut state.view_state {
            View::SubList(_posts, ref mut list_state) => {
                list_state.select(list_state.selected().map(|s| if s > 0 { s - 1 } else { s }));
            }
            View::PostView(_post_view, ref mut list_state) => {
                list_state.select(list_state.selected().map(|s| if s > 0 { s - 1 } else { s }));
            }
            View::Loading => {}
        },
        Msg::Quit => {
            bail!("Quitting"); // TODO: Better quiting path
        }
        Msg::Select => match &state.view_state {
            View::SubList(posts, list_state) => {
                if let Some(url) = list_state
                    .selected()
                    .and_then(|i| posts.get(i))
                    .map(|post| post.url.as_str())
                    .and_then(|s| s.strip_prefix("\""))
                    .and_then(|s| s.strip_suffix("\""))
                {
                    webbrowser::open(url)?;
                }
            }
            View::PostView(_, _) => todo!(),
            View::Loading => {}
        },
        Msg::Up => unimplemented!(),
        Msg::Down => match &state.view_state {
            View::SubList(posts, list_state) => {
                if let Some(permalink) =
                    list_state
                        .selected()
                        .and_then(|i| posts.get(i))
                        .and_then(|post| {
                            Some(
                                post.permalink
                                    .strip_suffix("\"")?
                                    .strip_prefix("\"")?
                                    .to_string(),
                            )
                        })
                {
                    state.view_state = View::Loading;
                    return Ok(Some(Box::pin(async move {
                        let comments = get_post_view(permalink.as_str()).await?;
                        Ok(Msg::CommentsResponse(comments))
                    })));
                }
            }
            View::PostView(_, _) => todo!(),
            View::Loading => {}
        },
        Msg::CommentsResponse(post_view) => {
            let mut list_state = ListState::default();
            if !post_view.comments.is_empty() {
                list_state.select(Some(0));
            }
            state.view_state = dbg!(View::PostView(post_view, list_state));
        }
    }
    Ok(None)
}

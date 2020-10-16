use crate::{api::get_post_view, api::get_posts, msg::Msg, state::State};
use anyhow::{bail, Result};
use futures::future::BoxFuture;
use termion::event::Key;
use tui::widgets::ListState;

pub fn update(
    msg: Msg,
    state_stack: &mut Vec<State>,
) -> Result<Option<BoxFuture<'static, Result<Msg>>>> {
    let last_state_index = state_stack.len() - 1;
    match (state_stack.as_mut_slice(), msg) {
        (_, Msg::Error(e)) => {
            bail!(e);
        }
        (_, Msg::FetchSubreddit(sub)) => {
            state_stack.push(State::Loading);
            return Ok(Some(Box::pin(async move {
                let posts = get_posts(sub.as_deref()).await?;
                Ok(Msg::SubredditResponse(posts, sub))
            })));
        }
        ([.., State::Loading], Msg::SubredditResponse(posts, sub)) => {
            let mut list_state = ListState::default();
            if !posts.is_empty() {
                list_state.select(Some(0));
            }
            state_stack[last_state_index] = State::SubList(posts, list_state, sub);
        }
        ([.., State::SubList(_, _, sub)], Msg::Input(Key::Char('r'))) => {
            return update(Msg::FetchSubreddit(sub.clone()), state_stack);
        }
        ([.., State::SubList(posts, ref mut list_state, _)], Msg::Input(Key::Char('j'))) => {
            list_state.select(list_state.selected().map(|s| {
                if s < posts.len() - 1 {
                    s + 1
                } else {
                    s
                }
            }));
        }
        ([.., State::PostView(post_view, ref mut list_state)], Msg::Input(Key::Char('j'))) => {
            list_state.select(list_state.selected().map(|s| {
                if s < post_view.comments.len() {
                    s + 1
                } else {
                    s
                }
            }));
        }
        ([.., State::SubList(_posts, ref mut list_state, _)], Msg::Input(Key::Char('k'))) => {
            list_state.select(list_state.selected().map(|s| if s > 0 { s - 1 } else { s }));
        }
        ([.., State::PostView(_post_view, ref mut list_state)], Msg::Input(Key::Char('k'))) => {
            list_state.select(list_state.selected().map(|s| if s > 0 { s - 1 } else { s }));
        }
        ([.., State::PostView(_, _)], Msg::Input(Key::Char('h'))) => {
            state_stack.pop();
        }
        ([.., State::SubList(posts, list_state, _)], Msg::Input(Key::Char('\n'))) => {
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
        ([.., State::SubList(posts, list_state, _)], Msg::Input(Key::Char('l'))) => {
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
                state_stack.push(State::Loading);
                return Ok(Some(Box::pin(async move {
                    let comments = get_post_view(permalink.as_str()).await?;
                    Ok(Msg::CommentsResponse(comments))
                })));
            }
        }
        ([.., State::Loading], Msg::CommentsResponse(post_view)) => {
            let mut list_state = ListState::default();
            if !post_view.comments.is_empty() {
                list_state.select(Some(0));
            }
            state_stack[last_state_index] = State::PostView(post_view, list_state);
        }
        ([.., State::SelectSubreddit(prompt)], Msg::Input(Key::Char('\n'))) => {
            return update(Msg::FetchSubreddit(Some(prompt.clone())), state_stack);
        }
        ([.., State::SelectSubreddit(_)], Msg::Input(Key::Esc)) => {
            state_stack.pop();
        }
        ([.., State::SelectSubreddit(ref mut prompt)], Msg::Input(Key::Backspace)) => {
            prompt.pop();
        }
        ([.., State::SelectSubreddit(ref mut prompt)], Msg::Input(Key::Char(c))) => {
            prompt.push(c);
        }
        (_, Msg::Input(Key::Char('q'))) => {
            bail!("Quitting"); // TODO: Better quiting path
        }
        (_, Msg::Input(Key::Ctrl('c'))) => {
            bail!("Quitting"); // TODO: Better quiting path
        }
        (_, Msg::Input(Key::Char('/'))) => {
            state_stack.push(State::SelectSubreddit(String::new()));
        }
        (_, _) => {}
    }
    Ok(None)
}

use crate::{
    api::get_posts,
    msg::Msg,
    state::{State, View},
};
use anyhow::{bail, Error, Result};
use flume::Sender;
use tui::widgets::ListState;
use futures::Future;

pub fn update(msg: Msg, state: &mut State, tx: Sender<Msg>) -> Result<Option<impl Future<Output=Result<()>>>> {
    match msg {
        Msg::FetchSubreddit(sub) => {
            state.view_state = View::Loading;
            return Ok(Some(async move {
                let posts = get_posts(sub.as_ref().map(|s| s.as_str())).await?;
                tx.send(Msg::SubredditResponse(posts)).map_err(Error::new)?;
                Ok(())
            }))
        }
        Msg::SubredditResponse(posts) => {
            let mut list_state = ListState::default();
            if posts.len() > 0 {
                list_state.select(Some(0));
            }
            state.view_state = View::SubList(posts, list_state);
        }
        Msg::Error(e) => {
            bail!(e);
        }
        Msg::Down => match &mut state.view_state {
            View::SubList(posts, ref mut list_state) => {
                list_state.select(list_state.selected().map(|s| {
                    if s < posts.len() - 1 {
                        s + 1
                    } else {
                        s
                    }
                }));
            }
            View::Loading => {},
        },
        Msg::Up => match &mut state.view_state {
            View::SubList(posts, ref mut list_state) => {
                list_state.select(list_state.selected().map(|s| {
                    if s > 0 {
                        s - 1
                    } else {
                        s
                    }
                }));
            }
            View::Loading => {},
        },
        Msg::Quit => {
            bail!("Quitting"); // TODO: Better quiting path
        }
    }
    Ok(None)
}

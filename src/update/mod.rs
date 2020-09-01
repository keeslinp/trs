use crate::{msg::Msg, state::{View, State}, api::get_posts};
use flume::Sender;
use futures::task::{Spawn, SpawnExt};
use anyhow::{Result, Error, bail};

pub fn update(msg: Msg, state: &mut State, tx: Sender<Msg>, pool: impl Spawn) -> Result<()> {
    match msg {
        Msg::FetchSubreddit(sub) => {
            pool.spawn(async move {
                if let Err(e) = get_posts(sub.as_str()).await.and_then(|posts| {
                    tx.send(Msg::SubredditResponse(posts)).map_err(Error::new)
                }) {
                    eprintln!("{:?}", e);
                    tx.send(Msg::Error("Failed to fetch subreddit posts".to_owned()));
                }
            })?;
            state.view_state = View::Loading;
        },
        Msg::SubredditResponse(posts) => {
            state.view_state = View::SubList(posts);
        },
        Msg::Error(e) => {
            eprintln!("error: {:?}", e);
        }
        Msg::Quit => {
            bail!("Quitting"); // TODO: Better quiting path
        }
    }
    Ok(())
}

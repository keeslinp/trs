use crate::{msg::Msg, state::{View, State}, api::get_posts};
use flume::Sender;
use futures::{future::RemoteHandle, task::{Spawn, SpawnExt}};
use anyhow::{Result, Error, bail};

pub fn update(msg: Msg, state: &mut State, tx: Sender<Msg>, pool: impl Spawn) -> Result<()> {
    match msg {
        Msg::FetchSubreddit(sub) => {
            let _: RemoteHandle<Result<()>> = pool.spawn_with_handle(async move {
                let posts = get_posts(sub.as_str()).await?;
                dbg!(posts);
                tx.send(Msg::SubredditResponse(posts)).map_err(Error::new)?;
                Ok(())
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

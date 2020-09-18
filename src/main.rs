use anyhow::Result;
use futures::executor::ThreadPool;
use futures::{
    future::RemoteHandle,
    task::{SpawnExt},
};
use std::io;
use termion::raw::IntoRawMode;
use tui::{backend::TermionBackend, Terminal};

mod api;
mod model;
mod msg;
mod render;
mod state;
mod update;

use msg::Msg;
use render::render;
use state::State;
use update::update;

fn handle_input(tx: flume::Sender<Msg>) {
    use std::thread::JoinHandle;

    let _: JoinHandle<Result<()>> = std::thread::spawn(move || {
        use termion::{event::Key, input::TermRead};
        let stdin = io::stdin();
        let lock = stdin.lock();
        for key in lock.keys() {
            match key? {
                Key::Char('q') => tx.send(Msg::Quit)?,
                Key::Char('j') => tx.send(Msg::Down)?,
                Key::Char('k') => tx.send(Msg::Up)?,
                Key::Char('\n') => tx.send(Msg::Select)?,
                _ => {}
            }
        }
        Ok(())
    });
}

fn main() -> Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut state = State::default();
    let (tx, rx) = flume::unbounded();
    let pool = ThreadPool::new()?;
    handle_input(tx.clone());
    tx.send(Msg::FetchSubreddit(None))?;
    terminal.clear()?;
    for msg in rx.iter() {
        let maybe_future = update(msg, &mut state, tx.clone())?;
        if let Some(future) = maybe_future {
            let handle: RemoteHandle<Result<()>> = pool.spawn_with_handle(future)?;
            handle.forget();
        }
        render(&mut terminal, &mut state);
    }
    Ok(())
}

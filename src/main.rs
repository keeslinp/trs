use anyhow::Result;
use std::io;
use termion::raw::IntoRawMode;
use tui::{backend::TermionBackend, Terminal};
use futures::executor::ThreadPool;

mod model;
mod msg;
mod render;
mod state;
mod update;
mod api;

use render::render;
use state::State;
use update::update;
use msg::Msg;

fn handle_input(tx: flume::Sender<Msg>) {
    use std::thread::JoinHandle;

    let _: JoinHandle<Result<()>> = std::thread::spawn(move || {
        use termion::{input::TermRead, event::Key};
        let stdin = io::stdin();
        let lock = stdin.lock();
        for key in lock.keys() {
            match key? {
                Key::Char('q') => tx.send(Msg::Quit)?,
                _ => {},
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
    tx.send(Msg::FetchSubreddit("rust".to_owned()))?;
    for msg in rx.iter() {
        update(msg, &mut state, tx.clone(), pool.clone())?;
        render(&mut terminal, &state);
    }
    Ok(())
}
